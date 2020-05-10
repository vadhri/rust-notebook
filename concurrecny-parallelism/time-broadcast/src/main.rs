use std::time::{Duration, Instant};
use crossbeam:: crossbeam_channel;
use clap::{clap_app, crate_version};

use crossbeam_channel::{after, tick};
use std::thread;

use crossbeam_channel::select;

pub fn main () {
    let args_handler = clap_app!(send_interrupt =>
        (version: crate_version!())
        (author: "Vadhri")
        (@arg interval: +required "Interval in which we need to send an interrupt.")
        (@arg time: +required "Time for how long the test should run.")
    ).get_matches();

    let mut interval: f64 = 0.0;
    let mut time: f64 = 0.0;

    interval = match (*args_handler.value_of("interval").unwrap()).parse::<f64>() {
        Ok(value) => value,
        Err(reason) => {
            panic!("Error {:?} -> Interval value should be an integer ! ({:?})", reason,
                args_handler.value_of("interval").unwrap());
        }
    };

    time = match (*args_handler.value_of("time").unwrap()).parse::<f64>() {
        Ok(value) => value,
        Err(reason) => {
            panic!("Error {:?} -> Time value should be an integer ! ({:?})", reason,
                args_handler.value_of("time").unwrap());
        }
    };
    let start = Instant::now();

    let timeout = after(Duration::from_millis(time as u64));

    let ticker = tick(Duration::from_millis(interval as u64));
    let timeout_main = after(Duration::from_millis(time as u64));

    let ticker_main = ticker.clone();
    let ticker_t2 = ticker.clone();
    let ticker_t1 = ticker.clone();

    let timeout_t2 = after(Duration::from_millis(time as u64));


    let t1 = thread::spawn(move || {
        loop {
            select! {
                recv(ticker_t1) -> _ => println!("T1  -> elapsed: {:?}", start.elapsed()),
                recv(timeout) -> _ => {
                    println!("T1 stopped");
                    break
                }
            }
        }
    });

    let t2 = thread::spawn(move || {
        loop {
            select! {
                recv(ticker_t2) -> _ => println!("T2  -> elapsed: {:?}", start.elapsed()),
                recv(timeout_t2) -> _ => {
                    println!("T2 stopped");
                    break
                }
            }
        }
    });

    loop {
        select! {
            recv(ticker_main) -> _ => println!("Main process => elapsed: {:?}", start.elapsed()),
            recv(timeout_main) -> _ => break
        }
    }

    let _ignore = t1.join();
    let _ignore = t2.join();
}
