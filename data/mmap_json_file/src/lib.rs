//! # mmap_json_file
//!
//! `mmap_json_file` is a collection of utilities to filter and count ( with and without filter )
//!
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
use std::{collections::HashMap, thread};

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

/// Filter the contents of a json file with filter specified and write the output to file specified.
///
/// # Arguments
///
/// * `f` Filename and full accesislbe path of the input json file
/// * `filter` A closure that can handle an input parameter of type T.
/// * `o` Filename and full accesislbe path of the output json file.
///
/// # Input types
///
/// * `T` The type of the structure.
/// * `F` Closure template with input function type.
///
/// # Example usage
///
/// ```
///
/// use mmap_json_file;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimple {
///     a: Option<String>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleNested {
///     b: Option<TestSimple>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleCompound {
///     a: Option<TestSimpleNested>,
///     f: Option<String>,
/// }
///
/// let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };
///
/// let _res = mmap_json_file::filter::<TestSimple, Box<dyn Fn(TestSimple) -> bool>>(
///     "data/test_simple.json".to_string(),
///     Box::new(filter),
///     "output.json".to_string(),
/// );
/// ```

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

/// Count the contents of a json file with filter specified.
///
/// # Arguments
///
/// * `f` Filename and full accesislbe path of the input json file
/// * `filter` A closure that can handle an input parameter of type T.
///
/// # Return
///
/// * `count` No of records that match the filter.
///
/// # Input types
///
/// * `T` The type of the structure.
/// * `F` Closure template with input function type.
///
/// # Example usage
///
/// ```
/// use mmap_json_file;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimple {
///     a: Option<String>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleNested {
///     b: Option<TestSimple>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleCompound {
///     a: Option<TestSimpleNested>,
///     f: Option<String>,
/// }
///
/// let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };
///
/// let _res = mmap_json_file::count_with_filter::<
///     TestSimple,
///    Box<dyn Fn(TestSimple) -> bool>,
/// >("data/test_simple.json".to_string(), Box::new(filter));
/// ```
///

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

/// Count the contents of a json file.
///
/// # Arguments
///
/// * `f` Filename and full accesislbe path of the input json file
///
/// # Return
///
/// * `count` No of records that match the filter.
///
/// # Input types
///
/// * `T` The type of the structure.
///
/// # Example usage
///
/// ```
/// use mmap_json_file;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimple {
///     a: Option<String>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleNested {
///     b: Option<TestSimple>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleCompound {
///     a: Option<TestSimpleNested>,
///     f: Option<String>,
/// }
///
/// let filter = |record: TestSimple| -> bool { record.a.unwrap() == "b" };
///
/// let _res = mmap_json_file::count_with_filter::<
///     TestSimple,
///    Box<dyn Fn(TestSimple) -> bool>,
/// >("data/test_simple.json".to_string(), Box::new(filter));
/// ```
///

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

/// Disctinct values of the contents of a json field.
///
/// * `f` Filename and full accesislbe path of the input json file
/// * `filter` A closure that can handle an input parameter of type T and provide the field with distincts.
/// * `o` Filename and full accesislbe path of the output json file.
///
/// # Input types
///
/// * `T` The type of the structure.
/// * `F` Closure template with input function type.
///
/// # Return
///
/// * `count` No of distinct records found.
///
/// # Example usage
///
/// ```
/// use mmap_json_file;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimple {
///     a: Option<String>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleNested {
///     b: Option<TestSimple>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleCompound {
///     a: Option<TestSimpleNested>,
///     f: Option<String>,
/// }
///
/// let filter = |record: TestSimple| -> String { record.a.unwrap() };
///
/// let _res = mmap_json_file::distinct_of_field::<
///     TestSimple,
///    Box<dyn Fn(TestSimple) -> String>,
/// >("data/test_simple.json".to_string(), Box::new(filter), "output".to_string());
/// ```
///

pub fn distinct_of_field<'a, T: 'static, F>(f: String, filter: F, o: String) -> Result<i32>
where
    T: DeserializeOwned + std::fmt::Debug + Clone + Send + serde::Serialize,
    F: Fn(T) -> String,
{
    let file = File::open(f).expect("failed to open the input file.");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
    let mut q = Vec::new();

    let (sender, receiver) = unbounded::<String>();
    let (sender_write_count, receiver_write_count) = bounded(1);

    let mut output = File::create(o.clone()).unwrap();
    let mut vect = Vec::new();
    let mut hm = HashMap::new();

    let writer = thread::spawn(move || -> Result<()> {
        while let record = receiver.recv() {
            match record {
                Err(_reason) => {
                    break;
                }
                Ok(value) => {
                    match hm.contains_key(&value) {
                        true  => {
                            // ignore the value
                        },
                        false => {
                            hm.insert(value, true);
                        }
                    }

                }
            }
        }

        serde_json::to_writer::<File, Vec<&String>>(output, &hm.keys().collect()).expect("Cannot write to destination");

        match sender_write_count.send(hm.keys().len() as i32) {
            Ok(_value) => Ok(()),
            Err(_reason) => {
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
                    let s = filter(deserialized.clone());

                    if s.len() > 0 {
                        sender.send(s).expect("Internal error!");
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
    Ok(receiver_write_count.recv().unwrap())
}
/// Sum values of a json field over the entire file.
///
/// * `f` Filename and full accesislbe path of the input json file
/// * `filter` A closure that can handle an input parameter of type T and provide the field with distincts.
///
/// # Input types
///
/// * `T` The type of the structure.
/// * `F` Closure template with input function type.
/// * `U` The type of the result. ( depending on the type of the field being summed over.)
///
/// # Return
///
/// * `sum` Total sum.
///
/// # Example usage
///
/// ```
/// use mmap_json_file;
/// use serde::{Deserialize, Serialize};
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimple {
///     a: Option<String>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleNested {
///     b: Option<TestSimple>,
///     c: Option<String>,
/// }
///
/// #[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// struct TestSimpleCompound {
///     a: Option<TestSimpleNested>,
///     f: Option<String>,
/// }
///
/// let filter = |record: TestSimple| -> f64 {
///    match record.a {
///        Some(value) => {
///            match value.parse::<f64>() {
///                Ok(num) => num as f64,
///                _ => 0f64
///            }
///        },
///        _ => 0f64
///    }
/// };
///
/// let _res = mmap_json_file::sum_over_field::<TestSimple, Box<dyn Fn(TestSimple) -> f64>, f64>(
///    "data/test_simple_sum.json".to_string(),
///    Box::new(filter)
/// );

pub fn sum_over_field<'a, T: 'static, F, U>(f: String, filter: F) -> Result<U>
where
    T: DeserializeOwned + std::fmt::Debug + Clone + Send + serde::Serialize,
    F: Fn(T) -> U,
    U: Default + std::ops::AddAssign
{
    let file = File::open(f).expect("failed to open the file");
    let mmap = unsafe { Mmap::map(&file).expect("failed to map the file") };
    let mut count:U = U::default();
    let mut vect = Vec::new();

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
                    count += filter(deserialized.clone());

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
