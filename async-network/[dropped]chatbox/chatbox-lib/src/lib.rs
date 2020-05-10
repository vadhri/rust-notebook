use core::fmt::Debug;
use futures::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use futures::channel::{oneshot, mpsc};

use futures::channel::mpsc::Receiver;
use futures::channel::mpsc::Sender;

#[derive(Debug)]
pub enum Command<T> {
    Receive(T),
    InquireNumberOfMessages(oneshot::Sender<u32>)
}

#[derive(Debug)]
pub struct ChatBox<T> {
    inbox: Vec<T>,
    listener: Receiver<Command<T>>
}

impl<T> ChatBox<T> {
    pub fn new() -> (Self, Sender<Command<T>>) {
        let (tx, rx) = mpsc::channel::<Command<T>>(10);

        (ChatBox {
            inbox: Vec::new(),
            listener: rx
        },
        tx.clone())
    }
}

impl<T> Future for ChatBox<T> where T: Debug + Clone + Unpin {
    type Output = Vec<T>;

    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        loop {
            match self.listener.try_next() {
                Ok(value) => {
                    match value.unwrap() {
                        Command::Receive(value) => {
                            self.inbox.push(value);
                        },
                        Command::InquireNumberOfMessages(tx) => {
                            tx.send(self.inbox.len() as u32).unwrap();
                        }
                    }

                },
                _rest => {
                    break
                }
            };
        }

        Poll::Ready(self.inbox.clone())
    }
}

#[cfg(test)]
mod tests {
    use futures::future::lazy;
    use super::*;
    use futures::executor;
    use futures::try_join;

    #[test]
    fn send_no_items() {
        let cb = ChatBox::<String>::new();

        let ft = async {
            let p = cb.0.await;

            assert_eq!(p.len(), 0);
        };

        executor::block_on(ft);
    }

    #[test]
    fn send_multiple_items() {
        let mut cb = ChatBox::<String>::new();

        let _res = cb.1.try_send(Command::Receive("100".to_string()));
        let _res = cb.1.try_send(Command::Receive("101".to_string()));
        let _res = cb.1.try_send(Command::Receive("102".to_string()));
        let _res = cb.1.try_send(Command::Receive("103".to_string()));
        let _res = cb.1.try_send(Command::Receive("104".to_string()));
        let _res = cb.1.try_send(Command::Receive("105".to_string()));

        let ft = async {
            let p = cb.0.await;

            assert_eq!(p.len(), 6);
        };

        executor::block_on(ft);
    }

    #[test]
    fn try_join() {
        let cb = ChatBox::<String>::new();

        let ft1 = async {
            let p = cb.0.await;

            assert_eq!(p.len(), 0);
            Ok::<i32, i32>(0)
        };

        let mut cb1 = ChatBox::<String>::new();

        let _res = cb1.1.try_send(Command::Receive("100".to_string()));

        let ft2 = async {
            let p = cb1.0.await;

            assert_eq!(p.len(), 1);
            Ok::<i32, i32>(0)
        };

        let combined = async {
            let _a = try_join!(ft1, ft2);
        };

        executor::block_on(combined);
    }

    #[test]
    fn try_lazy() {
        let mut cb = ChatBox::<u32>::new();

        let cbl = lazy(|_| {
            let _res = cb.1.try_send(Command::Receive(100));
        });

        executor::block_on(cbl);

        let ft = async {
            let p = cb.0.await;

            assert_eq!(p.len(), 1);
        };

        executor::block_on(ft);
    }

    #[test]
    fn try_one_shot_inquiry() {
        let mut cb = ChatBox::<String>::new();
        let (tx, mut rx) = oneshot::channel::<u32>();

        let _res = cb.1.try_send(Command::Receive("100".to_string()));
        let _res = cb.1.try_send(Command::Receive("101".to_string()));
        let _res = cb.1.try_send(Command::Receive("102".to_string()));
        let _res = cb.1.try_send(Command::Receive("103".to_string()));

        let _res = cb.1.try_send(Command::InquireNumberOfMessages(tx));

        let ft = async {
            let p = cb.0.await;
            assert_eq!(p, vec!["100","101","102","103"]);

            let number_of_messages = rx.try_recv().unwrap();
            assert_eq!(p.len(), number_of_messages.unwrap() as usize);
        };

        executor::block_on(ft);
    }
}
