use derive_more::{Display, From, Into};
use rand::{thread_rng, Rng};

/// The ID of a message type. See [here](https://remexre.xyz/stahlos/protocols/index.html#messages).
#[derive(Clone, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
pub struct MessageTypeID(u64);

/// The ID of a task.
#[derive(Clone, Copy, Debug, Display, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
pub struct TaskID(u64);

impl TaskID {
    /// Generates a random TaskID.
    pub fn rand() -> TaskID {
        TaskID(thread_rng().gen())
    }
}
