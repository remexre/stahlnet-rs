use derive_more::{Display, From, Into};
use rand::{thread_rng, Rng};

/// The ID of a message type. See [here](https://remexre.xyz/stahlos/protocols/index.html#messages).
#[derive(Clone, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
pub struct MessageTypeID(u64);

impl MessageTypeID {
    /// The [DebugPrint](https://remexre.xyz/stahlos/protocols/debug.html#debugprint) message type.
    pub const DEBUG_PRINT: MessageTypeID = MessageTypeID(0x66c379ab0b7196ef);

    /// The [Ping](https://remexre.xyz/stahlos/protocols/debug.html#ping) message type.
    pub const PING: MessageTypeID = MessageTypeID(0x0000000000000000);

    /// The [Pong](https://remexre.xyz/stahlos/protocols/debug.html#pong) message type.
    pub const PONG: MessageTypeID = MessageTypeID(0xffffffffffffffff);

    /// The [ReadBytesData](https://remexre.xyz/stahlos/protocols/byte-input-stream.html#readbytesdata) message type.
    pub const READ_BYTES_DATA: MessageTypeID = MessageTypeID(0x9d373588abfc316c);

    /// The [ReadBytes](https://remexre.xyz/stahlos/protocols/byte-input-stream.html#readbytes) message type.
    pub const READ_BYTES: MessageTypeID = MessageTypeID(0x453fbbee6bd6a904);

    /// The [ReadEOF](https://remexre.xyz/stahlos/protocols/byte-input-stream.html#readeof) message type.
    pub const READ_EOF: MessageTypeID = MessageTypeID(0x66822efd36030ee1);

    /// The [WriteBytesDone](https://remexre.xyz/stahlos/protocols/byte-output-stream.html#writebytesdone) message type.
    pub const WRITE_BYTES_DONE: MessageTypeID = MessageTypeID(0xfa2df296436c000b);

    /// The [WriteBytes](https://remexre.xyz/stahlos/protocols/byte-output-stream.html#writebytes) message type.
    pub const WRITE_BYTES: MessageTypeID = MessageTypeID(0xa80817401471655f);

    /// The [WriteEOF](https://remexre.xyz/stahlos/protocols/byte-output-stream.html#writeeof) message type.
    pub const WRITE_EOF: MessageTypeID = MessageTypeID(0xd43aa3d9caaf192e);
}

/// The ID of a task.
#[derive(Clone, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
pub struct TaskID(u64);

impl TaskID {
    /// Generates a random TaskID.
    pub fn rand() -> TaskID {
        TaskID(thread_rng().gen())
    }
}
