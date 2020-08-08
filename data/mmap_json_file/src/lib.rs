extern crate memmap;
use crossbeam;
use crossbeam::crossbeam_channel::bounded;
use crossbeam::crossbeam_channel::unbounded;
use failure;
use memmap::Mmap;
use serde;
use serde::de::DeserializeOwned;
use serde_json::Result;
use std::fs::File;
use std::io::prelude::*;
use std::thread;

use failure::Fail;

#[derive(Fail, Debug)]
pub enum ReturnValues {
    #[fail(display = "An internal error has occurred: {}", _0)]
    Success(i32),
    #[fail(display = "An internal error has occurred: {}", _0)]
    InternalChannelError(#[fail(cause)] crossbeam::crossbeam_channel::RecvError),
    #[fail(display = "An Io error has occurred: {}", _0)]
    Io(#[fail(cause)] std::io::Error),
    #[fail(display = "An json error has occurred: {}", _0)]
    Json(#[fail(cause)] serde_json::Error),
}

pub fn filter<'a, T: 'static + Default, F>(f: String, filter: F, o: String) -> Result<ReturnValues>
where
    T: DeserializeOwned + std::fmt::Debug + Clone + Send + serde::Serialize,
    F: Fn(T) -> bool,
{
    let file = File::open(f).expect("failed to open the input file.");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
    let mut q = Vec::new();

    let (sender, receiver) = unbounded();
    let (sender_write_count, receiver_write_count) = bounded(1);

    let mut output = File::create(o.clone()).unwrap();
    let mut vect = Vec::new();

    output
        .write("[".as_bytes())
        .expect("Cannot write to destination");

    let writer = thread::spawn(move || -> Result<()> {
        let mut write_count = 0;

        while let record = receiver.recv() {
            match record {
                Err(_reason) => {
                    break;
                }
                _ => match serde_json::to_string(&record.unwrap()) {
                    Ok(record_json_string) => match output.write(record_json_string.as_bytes()) {
                        Err(_reason) => {
                            break;
                        }
                        _ => {
                            write_count += 1;
                        }
                    },
                    Err(_reason) => {
                        break;
                    }
                },
            }
        }

        output
            .write("]".as_bytes())
            .expect("Cannot write to destination");

        match sender_write_count.send(write_count) {
            Ok(_value) => Ok(()),
            Err(reason) => {
                /* nothing to do with this error now! */
                Ok(())
            }
        }
    });

    for letter in mmap.iter() {
        match *letter as char {
            '}' => {
                q.pop();
                vect.push(*letter as char);

                if q.len() == 0 {
                    vect.remove(0);

                    let s: String = vect.clone().into_iter().collect::<String>();

                    let deserialized: T = serde_json::from_str(&s).unwrap();

                    if filter(deserialized.clone()) {
                        sender.send(deserialized).expect("Internal error!");
                    }

                    vect.clear();
                    q.clear();
                }
            },
            '\n' => {

            },
            '\t' => {

            },
            '{' => {
                vect.push(*letter as char);
                q.push(*letter as char);
            }
            _ => {
                vect.push(*letter as char);
            }
        }
    }

    drop(sender);

    let _msg = writer.join();

    Ok(ReturnValues::Success(receiver_write_count.recv().unwrap()))
}

pub fn count_with_filter<'a, T: 'static, F>(f: String, filter: F) -> Result<i32>
where
    T: DeserializeOwned + std::fmt::Debug + Clone + Send + serde::Serialize,
    F: Fn(T) -> bool,
{
    let file = File::open(f).expect("failed to open the file");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
    let mut vect = Vec::new();
    let mut count = 0;
    let mut q = Vec::new();

    for letter in mmap.iter() {
        match *letter as char {
            '}' => {
                q.pop();
                vect.push(*letter as char);

                if q.len() == 0 {
                    vect.remove(0);

                    let s: String = vect.clone().into_iter().collect::<String>();
                    let deserialized: T = serde_json::from_str(&s).unwrap();

                    if filter(deserialized.clone()) {
                        count += 1;
                    }

                    vect.clear();
                    q.clear();
                }
            },
            '\n' => {

            },
            '\t' => {

            },
            '{' => {
                vect.push(*letter as char);
                q.push(*letter as char);
            }
            _ => {
                vect.push(*letter as char);
            }
        }
    }

    Ok(count)
}

pub fn count<'a, T: 'static>(f: String) -> Result<i32>
where
    T: DeserializeOwned + std::fmt::Debug + Clone + Send + serde::Serialize
{
    let file = File::open(f).expect("failed to open the file");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
    let mut count = 0;

    let mut q = Vec::new();

    for letter in mmap.iter() {
        match *letter as char {
            '}' => {
                q.pop();
            },
            '{' => {
                if q.len() == 0 {
                    count += 1;
                    q.clear();
                }
                q.push(*letter as char);
            }
            _ => {}
        }
    }

    Ok(count)
}
