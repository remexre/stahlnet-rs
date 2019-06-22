use crate::{frame::Frame, MessageTypeID, TaskID};
use futures::{
    prelude::*,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};

/// A handle to the StahlNet server for communicating with it.
#[derive(Debug)]
pub struct TaskHandle {
    pub(crate) id: TaskID,
    pub(crate) recv: UnboundedReceiver<(TaskID, MessageTypeID, Vec<u8>)>,
    pub(crate) send: UnboundedSender<Frame>,
}

impl TaskHandle {
    /// Gets the task's ID.
    pub fn id(&self) -> TaskID {
        self.id
    }

    /// Sends a message.
    pub fn send(&self, to: TaskID, msg_type: MessageTypeID, data: Vec<u8>) -> Result<(), ()> {
        self.send
            .unbounded_send(Frame::Message(self.id, to, msg_type, data))
            .map_err(drop)
    }
}

impl Sink for TaskHandle {
    type SinkItem = (TaskID, MessageTypeID, Vec<u8>);
    type SinkError = ();

    fn start_send(
        &mut self,
        (to, msg_type, data): (TaskID, MessageTypeID, Vec<u8>),
    ) -> Result<AsyncSink<(TaskID, MessageTypeID, Vec<u8>)>, ()> {
        self.send
            .unbounded_send(Frame::Message(self.id, to, msg_type, data))
            .map(|()| AsyncSink::Ready)
            .map_err(drop)
    }

    fn poll_complete(&mut self) -> Result<Async<()>, ()> {
        self.send.poll_complete().map_err(drop)
    }
}

impl Stream for TaskHandle {
    type Item = (TaskID, MessageTypeID, Vec<u8>);
    type Error = ();

    fn poll(&mut self) -> Result<Async<Option<(TaskID, MessageTypeID, Vec<u8>)>>, ()> {
        self.recv.poll()
    }
}
