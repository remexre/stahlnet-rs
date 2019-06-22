use crate::{codec::Codec, frame::Frame, Error, MessageTypeID, TaskHandle, TaskID};
use futures::{
    future::{empty, ok},
    prelude::*,
    stream::{SplitSink, SplitStream},
    sync::mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
};
use libremexre::{errors::log_err, futures::send_to_sink};
use log::{error, trace};
use std::{
    collections::{HashMap, HashSet},
    fmt::{Debug, Formatter, Result as FmtResult},
    mem::replace,
    net::SocketAddr,
    time::{Duration, Instant},
};
use tokio::net::udp::{UdpFramed, UdpSocket};
use tokio_timer::Delay;

/// A network listener for the StahlNet server, plus some number of outgoing connections.
///
/// **WARNING**: This performs blocking I/O on drop unless the `.set_send_peer_bye(false)` method
/// is called.
pub struct Server {
    addr: SocketAddr,
    chan_recv: UnboundedReceiver<Frame>,
    chan_send: UnboundedSender<Frame>,
    peers: HashSet<SocketAddr>,
    queue_timeout: Duration,
    queued_messages: HashMap<TaskID, Vec<(Delay, TaskID, MessageTypeID, Vec<u8>)>>,
    routes: HashMap<TaskID, (u64, SocketAddr)>,
    send_peer_bye: bool,
    sock_recv: SplitStream<UdpFramed<Codec>>,
    sock_send_fut:
        Option<Box<dyn Future<Item = SplitSink<UdpFramed<Codec>>, Error = Error> + Send>>,
    shutdown: Box<dyn Future<Item = (), Error = ()> + Send>,
    tasks: HashMap<TaskID, UnboundedSender<(TaskID, MessageTypeID, Vec<u8>)>>,
}

impl Server {
    /// Creates a new server listening on the given address.
    pub fn new(
        addr: SocketAddr,
        shutdown: impl Future<Item = (), Error = ()> + Send + 'static,
    ) -> Result<Server, std::io::Error> {
        let sock = UdpSocket::bind(&addr)?;
        let addr = sock.local_addr()?;
        let sock = UdpFramed::new(sock, Codec);
        let (chan_send, chan_recv) = unbounded();
        let (sock_send, sock_recv) = sock.split();
        Ok(Server {
            addr,
            chan_recv,
            chan_send,
            peers: HashSet::new(),
            queue_timeout: Duration::from_secs(5),
            queued_messages: HashMap::new(),
            routes: HashMap::new(),
            send_peer_bye: true,
            sock_recv,
            sock_send_fut: Some(Box::new(ok(sock_send))),
            shutdown: Box::new(shutdown),
            tasks: HashMap::new(),
        })
    }

    /// Creates a new server listening on the default port (51441).
    pub fn new_from_defaults() -> Result<Server, std::io::Error> {
        Server::new(([0; 16], 51441).into(), empty())
    }

    /// Adds a peer to ask for tasks from.
    pub fn add_peer(&mut self, peer: SocketAddr) {
        self.and_then_send_and_flush(Frame::PeerHello, peer);
    }

    /// Adds another action on to the current action queue.
    fn and_then<
        Fun: FnOnce(SplitSink<UdpFramed<Codec>>) -> Fut + Send + 'static,
        Fut: Future<Item = SplitSink<UdpFramed<Codec>>, Error = Error> + Send + 'static,
    >(
        &mut self,
        func: Fun,
    ) {
        let fut = self.take_fut();
        self.sock_send_fut = Some(Box::new(fut.and_then(func)));
    }

    /// Adds a `flush` to the current action queue.
    fn and_then_flush(&mut self) {
        self.and_then(|sock| sock.flush());
    }

    /// Adds a `send_to_sink` to the current action queue.
    fn and_then_send(&mut self, frame: Frame, peer: SocketAddr) {
        self.and_then(move |sock| send_to_sink(sock, (frame, peer)));
    }

    /// Adds a `send_to_sink` and `flush` to the current action queue.
    fn and_then_send_and_flush(&mut self, frame: Frame, peer: SocketAddr) {
        self.and_then_send(frame, peer);
        self.and_then_flush();
    }

    /// Sends a frame to all peers.
    fn broadcast(&mut self, frame: Frame) {
        for peer in self.peers.clone() {
            let frame = frame.clone();
            self.and_then_send(frame, peer);
        }
        self.and_then_flush();
    }

    /// Handles an incoming frame from the given peer.
    fn handle_frame(&mut self, frame: Frame, peer: SocketAddr) {
        trace!("Got frame {:?}", frame);
        match frame {
            Frame::PeerHello => {
                debug_assert_ne!(peer, self.addr);
                if self.peers.insert(peer) {
                    self.and_then_send_and_flush(Frame::PeerHello, peer);
                }
            }
            Frame::PeerBye => {
                debug_assert_ne!(peer, self.addr);
                drop(self.peers.remove(&peer));
            }
            Frame::TaskHello(hops, task) => {
                debug_assert_ne!(peer, self.addr);
                if self
                    .routes
                    .get(&task)
                    .map(|&(known_hops, _)| hops <= known_hops)
                    .unwrap_or(true)
                {
                    drop(self.routes.insert(task, (hops, peer)));
                    self.broadcast(Frame::TaskHello(hops + 1, task));
                }
            }
            Frame::TaskBye(task) => {
                drop(self.queued_messages.remove(&task));
                let was_known =
                    self.routes.remove(&task).is_some() || self.tasks.remove(&task).is_some();
                if was_known {
                    self.broadcast(Frame::TaskBye(task));
                }
            }
            Frame::FindTask(task) => {
                debug_assert_ne!(peer, self.addr);
                if let Some((hops, _)) = self.routes.get(&task).cloned() {
                    self.and_then_send_and_flush(Frame::TaskHello(hops + 1, task), peer);
                }
            }
            Frame::Message(from, to, msg_type, data) => {
                self.send_message(from, to, msg_type, data);
            }
        }
    }

    /// Sends a message. This can be to and from a local or remote task.
    fn send_message(&mut self, from: TaskID, to: TaskID, msg_type: MessageTypeID, data: Vec<u8>) {
        if let Some(chan) = self.tasks.get(&to) {
            if let Err(err) = chan.unbounded_send((from, msg_type, data)) {
                let data = err.into_inner().2;
                self.send_message_remote(from, to, msg_type, data);
            }
        } else {
            self.send_message_remote(from, to, msg_type, data);
        }
    }

    /// Sends a message to a remote peer. This is somewhat non-trivial, as we have to find which
    /// peer the message should be sent to.
    fn send_message_remote(
        &mut self,
        from: TaskID,
        to: TaskID,
        msg_type: MessageTypeID,
        data: Vec<u8>,
    ) {
        if let Some((_, peer)) = self.routes.get(&to).cloned() {
            self.and_then_send_and_flush(Frame::Message(from, to, msg_type, data), peer);
        } else {
            self.broadcast(Frame::FindTask(to));
            let deadline = Instant::now() + self.queue_timeout;
            self.queued_messages.entry(to).or_default().push((
                Delay::new(deadline),
                from,
                msg_type,
                data,
            ));
        }
    }

    /// Sets whether to send a `PeerBye` frame on Drop or not. (Default `true`.)
    ///
    /// If this is `true`, dropping the Server will perform a *blocking* UDP send.
    pub fn set_send_peer_bye(&mut self, send_peer_bye: bool) {
        self.send_peer_bye = send_peer_bye;
    }

    /// Sets the amount of time a message can spend in the message queue while waiting to find a
    /// peer that can route to it. (Default 5 seconds.)
    pub fn set_queue_timeout(&mut self, queue_timeout: Duration) {
        self.queue_timeout = queue_timeout;
    }

    /// Spawns a task, returning a handle that can be used to refer to it.
    pub fn spawn_task(&mut self) -> TaskHandle {
        let id = TaskID::rand();
        let (send, recv) = unbounded();
        drop(self.tasks.insert(id, send));
        self.broadcast(Frame::TaskHello(0, id));
        TaskHandle {
            id,
            recv,
            send: self.chan_send.clone(),
        }
    }

    /// Takes the future for sending to the socket out from the Server.
    fn take_fut(
        &mut self,
    ) -> Box<dyn Future<Item = SplitSink<UdpFramed<Codec>>, Error = Error> + Send> {
        self.sock_send_fut
            .take()
            .expect("stahlnet::Server: Future is missing; did it panic?")
    }
}

impl Debug for Server {
    fn fmt(&self, fmt: &mut Formatter) -> FmtResult {
        fmt.debug_struct("Server")
            .field("addr", &self.addr)
            .field("peers", &self.peers)
            .field("queue_timeout", &self.queue_timeout)
            .field("queued_messages", &self.queued_messages)
            .field("routes", &self.routes)
            .field("send_peer_bye", &self.send_peer_bye)
            .field("tasks", &self.tasks)
            .finish()
    }
}

impl Drop for Server {
    fn drop(&mut self) {
        self.broadcast(Frame::PeerBye);
        let fut = replace(&mut self.sock_send_fut, None);
        if let Err(err) = fut.wait() {
            error!("While dropping a stahlnet::Server, {}", err);
        }
    }
}

impl Future for Server {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<()>, Error> {
        match self.shutdown.poll() {
            Ok(Async::Ready(())) => return Ok(Async::Ready(())),
            Ok(Async::NotReady) => {}
            Err(_) => {}
        }

        let mut fut = self.take_fut();
        self.sock_send_fut = match fut.poll() {
            Ok(Async::Ready(sock)) => Some(Box::new(ok(sock))),
            Ok(Async::NotReady) => Some(fut),
            Err(err) => return Err(err),
        };

        loop {
            match self.chan_recv.poll() {
                Ok(Async::Ready(Some(frame))) => {
                    // This is actually fairly wrong... However, since only Message and TaskBye
                    // frames are ever sent, it's okay...ish.
                    self.handle_frame(frame, self.addr);
                }
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(None)) | Err(()) => {
                    panic!("stahlnet::Server.chan_recv got a hangup")
                }
            }
        }

        loop {
            match self.sock_recv.poll() {
                Ok(Async::Ready(Some((frame, peer)))) => self.handle_frame(frame, peer),
                Ok(Async::NotReady) => break,
                Ok(Async::Ready(None)) => panic!("stahlnet::Server.sock_recv was closed"),
                Err(err) => log_err(&err),
            }
        }

        for (_, queue) in self.queued_messages.iter_mut() {
            // TODO: Once rust-lang/rust#43244 lands, this should use Vec::drain_filter.
            let mut i = 0;
            while i < queue.len() {
                let (delay, _, _, _) = &mut queue[i];
                if let Ok(Async::NotReady) = delay.poll() {
                    i += 1;
                } else {
                    drop(queue.remove(i));
                }
            }
        }

        self.queued_messages.retain(|_, queue| !queue.is_empty());

        Ok(Async::NotReady)
    }
}
