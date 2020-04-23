#![allow(dead_code)]

use core::fmt::Debug;

/* static dispatch */
pub fn print_value<T: Debug>(value: T) {
    println!("{:?}", value);
}

/* dynamic dispatch */
pub fn print_f64(x: &f64) {
    println!("print_f64 {:?}", x);
}

/* dynamic dispatch as box dyn */
trait Getabsolutevalue {
    fn compute(&self, num: f64) -> f64;
}
#[derive(Debug)]
struct PositiveNumber;
#[derive(Debug)]
struct NegativeNumber;

impl Getabsolutevalue for PositiveNumber {
    fn compute (&self, num: f64) -> f64 {
        println!("Getabsolutevalue -> PositiveNumber {:?}", num);

        num
    }
}

impl Getabsolutevalue for NegativeNumber {
    fn compute (&self, num: f64) -> f64 {
        println!("Getabsolutevalue -> NegativeNumber {:?}", num);
        0f64 - num
    }
}

pub fn dispatch_test () {
    let number = 100;
    let st = "Test";

    print_value(number);
    print_value(st);

    print_f64(&(100 as f64));
    print_f64(&(100.0 as f64));
    print_f64(&(-1 as f64));
    print_f64(&(-1.72 as f64));

    /* dynamic dispatch box dyn */
    let mut res= 0 as f64;

    for i in -20 .. 20 {
        let c = match i {
            x if x >= 0 => Box::new(PositiveNumber) as Box<dyn Getabsolutevalue>,
            _ => Box::new(NegativeNumber) as Box<dyn Getabsolutevalue>
        };
        res = c.compute(i as f64);
    }

    println!("{}", res);
}
