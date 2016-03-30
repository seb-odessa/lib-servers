//! This file contains types definitions

/// This trait guaranty that it's implementor has a name
pub trait HasName {
    /// Returns name of the implementor
    fn name(&self) -> String;
}

pub trait HasTarget {
    fn target(&self) -> Target;
}

pub trait Processor {
    fn process<T>(&self, arg: T) -> T;
}

pub enum Target {
    Processor,
    Consumer,
}

pub enum Message<T: HasName + HasTarget> {
    Quit,
    Request(T),
    Response(T),
    Busy(String),
    Free(String),
}
