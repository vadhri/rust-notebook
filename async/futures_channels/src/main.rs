extern crate futures;

use futures::channel::mpsc;
use futures::executor;
use futures::join;
use futures::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

#[derive(Debug, PartialEq)]
pub enum Operation {
    ADD,
    SUB,
}

#[derive(PartialEq, Debug)]
pub struct Calc {
    op: Operation,
    op1: u32,
    op2: u32,
    result: u32,
}

impl Future for Calc {
    type Output = u32;

    fn poll(mut self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<<Self as Future>::Output> {
        if self.op == Operation::ADD {
            self.result = self.op1 + self.op2;
        } else {
            self.result = self.op1 - self.op2;
        }
        Poll::Ready(self.result)
    }
}

fn main() {
    let (tx, mut rx) = mpsc::unbounded::<i32>();

    let ft = async {
        loop {
            let rec_future = rx.try_next();
            match rec_future {
                Ok(value) => {
                    println!("recv.. -> {:?}", value.unwrap());
                }
                _rest => {
                    break;
                }
            }
        }
        Ok::<i32, i32>(0)
    };

    let ft2 = async {
        for value in 0..10 {
            println!("sending.. -> {:?}", value);
            tx.unbounded_send(value as i32)
                .expect("Unexpected failure in sending");
        }

        Ok::<i32, i32>(0)
    };

    let combined = async {
        println!("Combined two futures .. ",);
        let _result = join!(ft2, ft);
    };

    let ft3 = async {
        let c = Calc {
            op: Operation::ADD,
            op1: 2,
            op2: 3,
            result: 0,
        };

        println!("{:?}", c.await);
    };

    executor::block_on(combined);
    executor::block_on(ft3);
}
