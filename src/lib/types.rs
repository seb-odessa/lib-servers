
pub trait Worker {
    fn name(&self)->String;
    fn process<T>(&self, data : T) -> T;
}

pub enum Message<Data:Send, Impl:Worker>{
    Nothing,
    Quit,
    Done(String),
    Request(String,Data),
    AddHandler(Impl),
}
