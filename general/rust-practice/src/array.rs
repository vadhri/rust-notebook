#![allow(dead_code)]
use std::mem;

pub fn array_test() {
    let a:[i32;5] = [0,1,2,3,4];
    let b:[i32;5] = [10,11,12,13,14];

    println!("a.length = {} head={}", a.len(), a[0]);
    println!("a full array = {:?}", a);

    if a != b {
        println!("a != b -> same array length")
    }

    println!("\n .. auto fill array .. ");
    let autofilled = [1;10];

    for i in 0..autofilled.len() {
        println!("b[{}] = {}", i, autofilled[i]);
    }

    println!("Array size b = {}", mem::size_of_val(&b));

    let matrix:[[i32;3];2] = [[1,2,3], [4,5,6]];

    println!("matrix = {:?}", matrix);
}
