use std::collections::VecDeque;
use std::sync::mpsc::{channel, Sender, Receiver};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct Queue<T> {
    queue: Arc<Mutex<VecDeque<T>>>,
    senders: Box<[Sender<bool>]>,
}

impl<T> Queue<T> {
    pub fn new(capacity: usize) -> (Self, Box<[Option<Receiver<bool>>]>) {
        let mut senders = Vec::<Sender<bool>>::with_capacity(capacity);
        let mut receivers = Vec::<Option<Receiver<bool>>>::with_capacity(capacity);

        for _ in 0..capacity {
            let (sender, receiver) = channel::<bool>();
            senders.push(sender);
            receivers.push(Some(receiver));
        }

        let queue = Queue::<T> {
            senders: senders.into_boxed_slice(),
            queue: Arc::new(Mutex::new(VecDeque::new())),
        };

        return (queue, receivers.into_boxed_slice());
    }

    pub fn push(&self, value: T) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(value);
        drop(queue);

        self.notify_listeners(true);
    }

    pub fn disconnect_all(&self) {
        self.notify_listeners(false);
    }

    pub fn recv(&self, receiver: &Receiver<bool>) -> Option<T> {
        loop {
            let mut queue = self.queue.lock().unwrap();
            let value_opt = queue.pop_front();
            drop(queue);

            if value_opt.is_some() {
                return Some(value_opt.unwrap());
            }

            let sig = receiver.recv();
            if sig.is_err() || sig.unwrap() == false {
                return None;
            }
        }
    }

    fn notify_listeners(&self, value: bool) {
        for sender in self.senders.iter() {
            let result = sender.send(value);
            if result.is_err() {
              return;
            }
            result.unwrap();
        }
    }
}
