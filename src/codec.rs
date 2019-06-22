use crate::{frame::Frame, Error};
use bytes::BytesMut;
use std::io::Cursor;
use tokio::codec::{Decoder, Encoder};

#[derive(Debug)]
pub struct Codec;

impl Decoder for Codec {
    type Item = Frame;
    type Error = Error;

    fn decode(&mut self, bytes: &mut BytesMut) -> Result<Option<Frame>, Error> {
        Frame::parse(Cursor::new(bytes)).map(Some)
    }
}

impl Encoder for Codec {
    type Item = Frame;
    type Error = Error;

    fn encode(&mut self, frame: Frame, bytes: &mut BytesMut) -> Result<(), Error> {
        frame.write_to_buf(bytes);
        Ok(())
    }
}
