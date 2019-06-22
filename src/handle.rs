use crate::{frame::Frame, Error, MessageTypeID, TaskID};
use futures::{
    prelude::*,
    sync::mpsc::{UnboundedReceiver, UnboundedSender},
};
use std::sync::Arc;

/// A handle to the StahlNet server for communicating with it.
#[derive(Debug)]
pub struct TaskHandle {
    pub(crate) id: TaskID,
    pub(crate) name: Option<Arc<str>>,
    pub(crate) recv: UnboundedReceiver<(TaskID, MessageTypeID, Vec<u8>)>,
    pub(crate) send: UnboundedSender<Frame>,
}

impl TaskHandle {
    /// Gets the task's ID.
    pub fn id(&self) -> TaskID {
        self.id
    }

    /// Gets the task's name.
    pub fn name(&self) -> &str {
        self.name.as_ref().map(|s| &**s).unwrap_or("")
    }

    /// Gets the task's name.
    pub fn name_arc(&self) -> Option<Arc<str>> {
        self.name.clone()
    }

    /// Sends a message.
    pub fn send_message(
        &self,
        to: TaskID,
        msg_type: MessageTypeID,
        data: Vec<u8>,
    ) -> Result<(), Error> {
        self.send
            .unbounded_send(Frame::Message(self.id, to, msg_type, data))
            .map_err(|_| Error::ShuttingDown)
    }
}

impl Drop for TaskHandle {
    fn drop(&mut self) {
        drop(self.send.unbounded_send(Frame::TaskBye(self.id)));
    }
}

impl Sink for TaskHandle {
    type SinkItem = (TaskID, MessageTypeID, Vec<u8>);
    type SinkError = Error;

    fn start_send(
        &mut self,
        (to, msg_type, data): (TaskID, MessageTypeID, Vec<u8>),
    ) -> Result<AsyncSink<(TaskID, MessageTypeID, Vec<u8>)>, Error> {
        self.send
            .unbounded_send(Frame::Message(self.id, to, msg_type, data))
            .map(|()| AsyncSink::Ready)
            .map_err(|_| Error::ShuttingDown)
    }

    fn poll_complete(&mut self) -> Result<Async<()>, Error> {
        self.send.poll_complete().map_err(|_| Error::ShuttingDown)
    }
}

impl Stream for TaskHandle {
    type Item = (TaskID, MessageTypeID, Vec<u8>);
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Option<(TaskID, MessageTypeID, Vec<u8>)>>, Error> {
        self.recv.poll().map_err(|_| Error::ShuttingDown)
    }
}
