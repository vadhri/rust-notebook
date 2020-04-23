#![allow(dead_code)]

pub fn closures_test() {
    let square_funcitons = |x: i32| -> i32 {
        x * x
    };

    println!("Square a variable - closures {:?}", square_funcitons(10));
}
