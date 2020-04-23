#![allow(dead_code)]

#[derive(Debug)]
struct Person<'lifetime> {
    name: &'lifetime str
}

#[derive(Debug)]
struct Company<'lifetime> {
    employees: Vec<Person<'lifetime>>
}

pub fn lifetime_test() {
    let p1 = Person {
        name: "Vadhri"
    };

    let p2 = Person {
        name: "Venkata"
    };

    let p3 = Person {
        name: "Ratnam"
    };

    let c = Company {
        employees: vec! [p1, p2, p3]
    };

    println!("Company => {:?}", c);
}
