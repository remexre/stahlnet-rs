use crate::{Error, MessageTypeID, TaskID};
use bytes::{Buf, BufMut};

#[derive(Clone, Debug)]
pub enum Frame {
    PeerHello,
    PeerBye,
    TaskHello(u64, TaskID),
    TaskBye(TaskID),
    FindTask(TaskID),
    Message(TaskID, TaskID, MessageTypeID, Vec<u8>),
}

impl Frame {
    pub fn parse(mut buf: impl Buf) -> Result<Frame, Error> {
        if buf.has_remaining() {
            let frame = match buf.get_u8() {
                0 => Frame::PeerHello,
                1 => Frame::PeerBye,
                2 => {
                    let hops = buf.get_u64_le();
                    let task = buf.get_u64_le();
                    Frame::TaskHello(hops, task.into())
                }
                3 => Frame::TaskBye(buf.get_u64_le().into()),
                4 => Frame::FindTask(buf.get_u64_le().into()),
                5 => {
                    let from = buf.get_u64_le();
                    let to = buf.get_u64_le();
                    let msg_type = buf.get_u64_le();
                    let mut data = vec![0; buf.remaining()];
                    buf.copy_to_slice(&mut data);
                    assert!(!buf.has_remaining());
                    Frame::Message(from.into(), to.into(), msg_type.into(), data)
                }
                n => return Err(Error::UnknownFrameType(n)),
            };
            if buf.has_remaining() {
                Err(Error::UnexpectedTrailing)
            } else {
                Ok(frame)
            }
        } else {
            Err(Error::UnexpectedEof)
        }
    }

    pub fn write_to_buf(&self, mut buf: impl BufMut) {
        match self {
            Frame::PeerHello => buf.put_u8(0x00),
            Frame::PeerBye => buf.put_u8(0x01),
            Frame::TaskHello(hops, task) => {
                buf.put_u8(0x02);
                buf.put_u64_le(*hops);
                buf.put_u64_le((*task).into());
            }
            Frame::TaskBye(task) => {
                buf.put_u8(0x03);
                buf.put_u64_le((*task).into());
            }
            Frame::FindTask(task) => {
                buf.put_u8(0x04);
                buf.put_u64_le((*task).into());
            }
            Frame::Message(from, to, msg_type, data) => {
                buf.put_u8(0x05);
                buf.put_u64_le((*from).into());
                buf.put_u64_le((*to).into());
                buf.put_u64_le((*msg_type).into());
                buf.put(data);
            }
        }
    }
}
