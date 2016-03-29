

pub trait HasName {
    fn name(&self)->String;
}

pub trait HasTarget {
    fn target(&self)->Target;
}
pub trait Processor {
    fn process<T>(&self, arg : T) -> T;
}

pub enum Target {
    Processor,
    Consumer,
}

pub enum Message<T :HasName + HasTarget>{
    Quit,
    Request(T),
    Response(T),
    Busy(String),
    Free(String),
}
