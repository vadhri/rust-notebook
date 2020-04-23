#![allow(dead_code)]

pub fn square_cube_quardraple(a: i32) -> (i32, i32,i32) {
    (a.pow(2), a.pow(3), a.pow(4))
}

fn tuples_print(t: (i32, i32, i32)) {
    println!("tuples_print -> t(0) -> {}, t(1) -> {}, t(2) -> {}", t.0, t.1, t.2);
}

pub fn tuples_test() {
    let t = square_cube_quardraple(4);

    println!("Tuples -> {:?}", t);
    tuples_print(t);

    // match tuples
    match t {
        (_, 256, _) => println!("Matched => t(0) -> {}, t(1) -> {}, t(2) -> {}", t.0, t.1, t.2),
        (0, 0, 0) => println!("Oh! Its empty!"),
        _ => println!("Default match")
    }
}
