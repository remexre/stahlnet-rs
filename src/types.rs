use derive_more::{Display, From, Into};
use rand::{thread_rng, Rng};

/// The ID of a message type. See [here](https://remexre.xyz/stahlos/protocols/index.html#messages).
#[derive(Clone, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
#[display(fmt = "{:x}", _0)]
pub struct MessageTypeID(u64);

impl MessageTypeID {
    /// The [DebugPrint](https://remexre.xyz/stahlos/protocols/debug.html#debugprint) message type.
    pub const DEBUG_PRINT: MessageTypeID = MessageTypeID(0x66c379ab0b7196ef);

    /// The [Notification](https://remexre.xyz/stahlos/protocols/notify.html#notification) message type.
    pub const NOTIFICATION: MessageTypeID = MessageTypeID(0xcb832fc8958bf9e6);

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

    /// Returns the MessageTypeID corresponding to the given hashed name.
    ///
    /// ```
    /// # use stahlnet::MessageTypeID;
    /// assert_eq!(MessageTypeID::DEBUG_PRINT, MessageTypeID::from_name("DebugPrint"));
    /// ```
    pub fn from_name(name: &str) -> MessageTypeID {
        // TODO: Once rust-lang/rust#57349 lands, this should become a const fn.
        let mut hash = 0xcbf29ce484222325;
        for b in name.bytes() {
            hash ^= b as u64;
            hash = hash.wrapping_mul(1099511628211);
        }
        MessageTypeID(hash)
    }
}

/// The ID of a task.
#[derive(Clone, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
#[display(fmt = "{:x}", _0)]
pub struct TaskID(u64);

impl TaskID {
    /// Generates a random TaskID.
    pub fn rand() -> TaskID {
        TaskID(thread_rng().gen())
    }
}
