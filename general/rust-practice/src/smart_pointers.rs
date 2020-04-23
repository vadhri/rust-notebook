#![allow(dead_code)]
use core::fmt::Display;
use std::ops::{Deref, DerefMut};
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Container<T> {
    width: T,
    height: T,
    depth: T
}

#[derive(Debug, PartialEq)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T: std::fmt::Display> Display for Container<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.width, self.height, self.depth)
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        // TODO: Cannot access values for drop trait on self.
        println!("Dropping CustomSmartPointer with data!");
    }
}

impl <T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

pub fn smart_pointers_passon<T: std::fmt::Debug>(input: Box<Container<T>>) {
    println!("{:?}", input);
}

pub fn smart_pointers_test() {
    let x = 6;
    let y = &x;

    println!("{} {}",x, *y);

    let b = Box::new(5);
    println!("Heap value -> {:?}", b);

    let bcontainer_1 = MyBox::new(Container {width: 10.0, height: 10.0, depth: 10.0});
    let bcontainer_2 = MyBox::new(Container {width: 10.0, height: 10.0, depth: 10.0});

    println!("Continer eq -> {:?}", bcontainer_1 == bcontainer_2);

    println!("Access values -> {} {} {}", bcontainer_1.width, bcontainer_1.height, bcontainer_1.depth);
    println!("Access values -> {:?}", bcontainer_1);
    println!("Access values with deref -> {:?}", *bcontainer_1.deref());

    let mut bcontainer_3 = MyBox::new(Container {width: 10.0, height: 10.0, depth: 10.0});
    let out: &mut Container<f32> = bcontainer_3.deref_mut();

    println!("{} {} {}", out.width, out.height, out.depth);

    out.width = 200 as f32;

    println!("{} {} {}", out.width, out.height, out.depth);
}
