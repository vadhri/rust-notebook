#![allow(dead_code)]

pub fn vectors_test() {
    let mut v = Vec::new();

    v.push(23);
    v.push(24);
    v.push(25);

    println!("{:?}", v);

    // add elements at index.
    v.insert(2, 41);

    println!("{:?}", v);

    // remove at index.
    v.remove(2);

    println!("{:?}", v);

    // length of vector
    println!("vec len = {}", v.len());

    // iterate vectors
    for i in 0..v.len() {
        println!("v[{}] = {}", i, v[i]);
    }

    println!(".. vec match with options ..");

    let idex = 1;

    match v.get(idex) {
        Some(x) => println!("v[{}] = {}", idex, x),
        None => println!("No such element !")
    }

    v.pop();

    println!("popped an element at end .. {:?}", v);

    let mut v1 = vec![1; 5];

    for i in &v1 {
        println!("{}", i);
    }

    println!("change vector elements in place .. ");

    for i in v1.iter_mut() {
        *i = 10;
    }

    println!("{:?}", v1);
}
