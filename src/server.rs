use crate::{codec::Codec, frame::Frame, Error, MessageTypeID, TaskHandle, TaskID};
use derivative::Derivative;
use futures::{
    future::ok,
    prelude::*,
    stream::{SplitSink, SplitStream},
    sync::{
        mpsc::{unbounded, UnboundedReceiver, UnboundedSender},
        oneshot::{channel, Receiver, Sender},
    },
};
use libremexre::{errors::log_err, futures::send_to_sink};
use log::trace;
use std::{
    collections::{HashMap, HashSet},
    net::SocketAddr,
    time::{Duration, Instant},
};
use tokio::net::udp::{UdpFramed, UdpSocket};
use tokio_timer::Delay;

/// A network listener for the StahlNet server, plus some number of outgoing connections.
///
/// **WARNING**: This performs blocking I/O on drop unless the `.set_send_peer_bye(false)` method
/// is called.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct Server {
    addr: SocketAddr,
    chan_recv: UnboundedReceiver<Frame>,
    chan_send: UnboundedSender<Frame>,
    peers: HashSet<SocketAddr>,
    queued_messages: HashMap<TaskID, Vec<(Delay, TaskID, MessageTypeID, Vec<u8>)>>,
    routes: HashMap<TaskID, (u64, SocketAddr)>,
    send_peer_bye: bool,
    sock_recv: SplitStream<UdpFramed<Codec>>,
    #[derivative(Debug = "ignore")]
    sock_send_fut:
        Option<Box<dyn Future<Item = SplitSink<UdpFramed<Codec>>, Error = Error> + Send>>,
    stop_recv: Receiver<()>,
    stop_send: Sender<()>,
    tasks: HashMap<TaskID, UnboundedSender<(TaskID, MessageTypeID, Vec<u8>)>>,
}

impl Server {
    /// Creates a new server listening on the given address.
    pub fn new(addr: SocketAddr) -> Result<Server, std::io::Error> {
        let sock = UdpSocket::bind(&addr)?;
        let sock = UdpFramed::new(sock, Codec);
        let (chan_send, chan_recv) = unbounded();
        let (sock_send, sock_recv) = sock.split();
        let (stop_send, stop_recv) = channel();
        Ok(Server {
            addr,
            chan_recv,
            chan_send,
            peers: HashSet::new(),
            queued_messages: HashMap::new(),
            routes: HashMap::new(),
            send_peer_bye: true,
            sock_recv,
            sock_send_fut: Some(Box::new(ok(sock_send))),
            stop_recv,
            stop_send,
            tasks: HashMap::new(),
        })
    }

    /// Creates a new server listening on the default port (51441).
    pub fn new_from_defaults() -> Result<Server, std::io::Error> {
        Server::new_from_port(51441)
    }

    /// Creates a new server listening on the given port.
    pub fn new_from_port(port: u16) -> Result<Server, std::io::Error> {
        Server::new(SocketAddr::from(([0u8; 16], port)))
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
                if self.peers.insert(peer) {
                    self.and_then_send_and_flush(Frame::PeerHello, peer);
                }
            }
            Frame::PeerBye => {
                drop(self.peers.remove(&peer));
            }
            Frame::TaskHello(hops, task) => {
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
            let deadline = Instant::now() + Duration::from_secs(5); // TODO: Make it configurable.
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

impl Future for Server {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<()>, Error> {
        match self.stop_recv.poll() {
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

        // TODO: Clear stale queued messages.

        Ok(Async::NotReady)
    }
}
