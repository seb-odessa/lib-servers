use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use super::types::{Message, Processor, HasName, HasTarget};

pub struct WorkerHandler<T: HasName + HasTarget, W: HasName + Processor> {
    gate: Sender<Message<T>>,
    input: Receiver<Message<T>>,
    output: Sender<Message<T>>,
    jobs: usize,
    worker: W,
}

impl<T: HasName + HasTarget, W: HasName + Processor> Drop for WorkerHandler<T, W> {
    fn drop(&mut self) {
        trace!("{} dropped. Processed {} tasks.",
               self.worker.name(),
               self.jobs);
    }
}

impl<T: HasName + HasTarget, W: HasName + Processor> WorkerHandler<T, W> {
    pub fn new(worker: W, output: Sender<Message<T>>) -> Self {
        trace!("WorkerHandler::new({}, ...)", &worker.name());
        let (tx, rx) = mpsc::channel();
        WorkerHandler {
            gate: tx,
            input: rx,
            output: output,
            jobs: 0,
            worker: worker,
        }
    }

    pub fn gate(&self) -> Sender<Message<T>> {
        self.gate.clone()
    }

    fn say(&self, msg: Message<T>) -> bool {
        return self.output.send(msg).is_ok();
    }

    pub fn run(&mut self) {
        while let Ok(msg) = self.input.recv() {
            match msg {
                Message::Quit => {
                    trace!("{} <= Message::Quit", self.worker.name());
                    break;
                }
                Message::Request(request) => {
                    self.jobs += 1;
                    let name = request.name();
                    trace!("{} <= Message::Request({}); {}",
                           self.worker.name(),
                           name,
                           self.jobs);
                    let ok = self.say(Message::Busy(name.clone())) &&
                             self.say(Message::Response(self.worker.process(request))) &&
                             self.say(Message::Free(name.clone()));
                    if !ok {
                        break;
                    }
                }
                _ => {
                    warn!("{} <= Unexpected message!!!", self.worker.name());
                }
            }
        }
    }
}
