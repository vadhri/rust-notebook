#![allow(dead_code)]

pub fn take_ownership_read(v: &Vec<i32>) {
    println!("owner take_ownership_read : argument {:?}", v);
}

pub fn take_ownership_mut(v: &mut Vec<i32>) {
    println!("owner take_ownership_mut : argument {:?}", v);
    v.push(100);
}

pub fn is_prime_number(i: u32) -> bool {
    let mut is_prime = true;
    for x in 2..i {
        if i % x == 0 {
            is_prime = false;
            break;
        }
    }
    is_prime
}

pub fn nth(n: u32) -> u32 {
    println!("Nth prime = {:?}", n  + 1);
    let mut prime_number: u32 = 2;
    let mut prime_numbers: Vec<u32> = Vec::new();

    while prime_numbers.len() < (n as usize) + 1 {
        if is_prime_number(prime_number) {
            prime_numbers.push(prime_number);
        }
        prime_number += 1;
    }

    let result = match prime_numbers.pop() {
        Some(x) => x,
        None => 0
    };

    result
}


pub fn ownership_test() {
    let mut v = Vec::new();

    v.push(1);
    v.push(2);

    println!("{:?}", v);

    take_ownership_read(&v);
    v.push(3);
    take_ownership_mut(&mut v);
    v.push(5);

    println!("{:?}", nth(10000));

    //is_prime_number(5);
}
