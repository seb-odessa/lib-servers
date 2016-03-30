//! This file contains types definitions

/// This trait guaranty that it's implementer has a name() function
pub trait HasName {
    /// Returns name of the implementer
    fn name(&self) -> String;
}

/// This trait guaranty that it's implementer has a target() function
pub trait HasTarget {
    /// Returns target of the implementer
    fn target(&self) -> String;
}

/// This trait guaranty that it's implementer has a process<T>() function
pub trait Processor {
    /// Takes arg of type <T>
    /// Returns result of the same type
    fn process<T>(&self, arg: T) -> T;
}

/// Message type used as container for all Supevisors/Workers communications
#[derive(Debug, PartialEq)]
pub enum Message<T: HasName + HasTarget> {
    /// Finish the work (function run()) and be ready to thread join
    Quit,
    /// Contains a portion of data for processing by the Worker
    /// T must has name and target
    Event(T),
    /// Inform Supervisor that worker become busy
    Busy(String),
    /// Inform Supervisor that worker become free for next event
    Free(String),
}
