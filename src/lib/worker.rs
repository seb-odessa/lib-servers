use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use super::types::{Message, Worker};

pub struct WorkerHandler<Data:Send, Impl:Worker>
{
    name : String,
    gate : Sender<Message<Data, Impl>>,
    input : Receiver<Message<Data, Impl>>,
    output  : Sender<Message<Data, Impl>>,
    processed : usize,
    workers : Vec<Impl>,
}
impl <Data:Send, Impl:Worker> Drop for WorkerHandler <Data, Impl> {
    fn drop(&mut self) {
        trace!("{} dropped. Processed {} tasks.", self.name, self.processed);
    }
}
impl <Data:Send, Impl:Worker> WorkerHandler <Data, Impl> {
    pub fn new<Name : Into<String>>(name:Name, results:Sender<Message<Data, Impl>>) -> Self {
        let name = name.into();
        trace!("{} created.", &name);
        let (tx, rx)  = mpsc::channel();
        WorkerHandler {
            name:name,
            gate:tx,
            input:rx,
            output:results,
            processed : 0,
            workers : Vec::new()
        }
    }

    pub fn gate(&self) -> Sender<Message<Data, Impl>> {
        self.gate.clone()
    }

    pub fn run(&mut self) {
        let mut done = false;
        while let Ok(msg) = self.input.recv() {
            match msg {
                Message::Nothing => {
                    trace!("{} <= Message::Nothing", self.name);
                },
                Message::Quit => {
                    trace!("{} <= Message::Quit", self.name);
                    done = true;
                },
                Message::AddHandler(worker) => {
                    trace!("{} <= Message::AddHandler({})", self.name, worker.name());
                    self.workers.push(worker);
                }
                Message::Request(target, data) => {
                    self.processed += 1;
                    trace!("{} <= Message::Request({}); {}", self.name, target, self.processed);
                    let mut current = data;
                    for worker in &self.workers {
                        current = worker.process(current);
                    }
                }
                _ => {
                    panic!("{} has received unexpected command.", self.name);
                }
            }
            self.output.send(Message::Done(self.name.clone())).unwrap();
            if done { break }
        }
    }
}
