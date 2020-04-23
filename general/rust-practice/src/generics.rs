#![allow(dead_code)]
use std::ops::Add;
use std::ops::Sub;
use std::ops::Div;

struct Calculator<T> {
    x: T,
    y: T
}

impl<T> Calculator<T> where T: Add<Output = T> + Copy + Sub<Output = T> + Div<Output = T> {
    pub fn add(&self) -> T {
        self.x + self.y
    }
    pub fn sub(&self) -> T {
        self.x - self.y
    }
    pub fn div(&self) -> T {
        self.x / self.y
    }
}

pub fn find_largest_in_vec<T: PartialOrd + Copy>(l: Vec<T>) -> T {
    let mut largest_value:T = l[0];

    for &item in l.iter() {
        if largest_value < item {
            largest_value = item;
        }
    }

    largest_value
}

pub fn generics_test() {
    let l = vec![10,14,1,99,76];
    println!("Largest value in int list -> {:?}", find_largest_in_vec(l));

    let l = vec!['a','z','m','r','l'];
    println!("Largest value in char list -> {:?}", find_largest_in_vec(l));

    let l = vec!["this", "is", "that", "zee"];
    println!("Largest value in char list -> {:?}", find_largest_in_vec(l));

    let op = Calculator { x: 1, y: 2 };
    println!("Generic struct impl add -> {}", op.add());
    println!("Generic struct impl sub -> {}", op.sub());
}
