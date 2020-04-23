#![allow(dead_code)]

pub fn options_test(x: f32, y: f32) {
    let result = if y != 0.0 { Some(x/y) } else { None };

    match result {
        Some(value) => println!("result = {} !!", value),
        None => println!("Never divide by zero !!!")
    }
}
