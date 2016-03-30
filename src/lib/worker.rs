//! Contains implementation of almost all Worker except process() function

use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use super::types::{Message, Processor, HasName, HasTarget};

/// WorkerHandler
pub struct WorkerHandler<T: HasName + HasTarget, W: HasName + Processor> {
    gate: Sender<Message<T>>,
    input: Receiver<Message<T>>,
    output: Sender<Message<T>>,
    received: usize,
    processed: usize,
    worker: W,
}

impl<T: HasName + HasTarget, W: HasName + Processor> Drop for WorkerHandler<T, W> {
    fn drop(&mut self) {
        trace!("{} dropped. Received {} tasks, processed {} tasks.",
               self.worker.name(),
               self.received,
               self.processed);
    }
}

impl<T: HasName + HasTarget, W: HasName + Processor> WorkerHandler<T, W> {
    /// WorkerHandler constructor
    pub fn new(worker: W, output: Sender<Message<T>>) -> Self {
        trace!("WorkerHandler::new({}, ...)", &worker.name());
        let (tx, rx) = mpsc::channel();
        WorkerHandler {
            gate: tx,
            input: rx,
            output: output,
            received : 0,
            processed: 0,
            worker: worker,
        }
    }

    /// Return the WorkerHandler's gate for managing of it
    pub fn gate(&self) -> Sender<Message<T>> {
        self.gate.clone()
    }

    fn say(&self, msg: Message<T>) -> bool {
        return self.output.send(msg).is_ok();
    }

    /// Runs instance of WorkerHandler. Should be launched in separate thread
    pub fn run(&mut self) {
        while let Ok(msg) = self.input.recv() {
            self.received += 1;
            match msg {
                Message::Quit => {
                    trace!("{} <= Message::Quit", self.worker.name());
                    break;
                }
                Message::Event(request) => {
                    let name = request.name();
                    trace!("{} <= Message::Request({})",
                           self.worker.name(),
                           name);

                    let succ = self.say(Message::Busy(name.clone())) &&
                               self.say(Message::Event(self.worker.process(request))) &&
                               self.say(Message::Free(name.clone()));
                    if !succ {
                        break;
                    }
                    self.processed += 1;
                    trace!("{} <= Message::Request({}); processed: {}",
                           self.worker.name(),
                           name,
                           self.processed);
                }
                _ => {
                    warn!("{} <= Unexpected message!!!", self.worker.name());
                }
            }
        }
        trace!("{} Has finished run()", self.worker.name());
    }
}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::mpsc::{Sender, Receiver};
    use std::thread;
    use types::{Message, Processor, HasName, HasTarget};
    use super::WorkerHandler;

    #[derive(Debug, PartialEq)]
    struct EventFake;
    impl HasName for EventFake
    {
        fn name(&self) -> String {String::from("EventFake")}
    }
    impl HasTarget for EventFake
    {
        fn target(&self) -> String {String::from("EventFakeTarget")}
    }

    #[derive(Debug, PartialEq)]
    struct TaskFake;
    impl HasName for TaskFake
    {
        fn name(&self) -> String {String::from("EventFake")}
    }
    impl Processor for TaskFake
    {
        fn process<T>(&self, event : T) -> T { event }
    }

    // impl PartialEq <EventFake> for Message<EventFake> {
    //     fn eq(&self, other: &EventFake) -> bool {
    //
    //     }
    //
    //     fn ne(&self, other: &EventFake) -> bool {
    //         !self.eq(other)
    //     }
    // }

    #[test]
    fn message_quit() {
        let task = TaskFake;
        let (pipe, _) : (Sender<Message<EventFake>>, Receiver<Message<EventFake>>) = mpsc::channel();
        let mut handler = WorkerHandler::new(task, pipe.clone());
        let gate = handler.gate();
        let thread = thread::spawn(move || handler.run());
        gate.send(Message::Quit).unwrap();
        thread.join().unwrap();
    }

    #[test]
    fn message_event() {
        let task = TaskFake;
        let taskname = task.name();
        let (pipe, results) : (Sender<Message<EventFake>>, Receiver<Message<EventFake>>) = mpsc::channel();
        let mut handler = WorkerHandler::new(task, pipe.clone());
        let gate = handler.gate();
        let thread = thread::spawn(move || handler.run());
        gate.send(Message::Event(EventFake)).unwrap();
        assert!(results.recv().unwrap() == Message::Busy(taskname.clone()));
        assert!(results.recv().unwrap() == Message::Event(EventFake));
        assert!(results.recv().unwrap() == Message::Free(taskname.clone()));
        gate.send(Message::Quit).unwrap();
        thread.join().unwrap();
    }

}
