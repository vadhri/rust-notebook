#![allow(dead_code)]

use std::collections::HashMap;
use std::collections::BTreeMap;
use std::fs;
use std::iter::FromIterator;

pub type MyHash = BTreeMap<String, i32>;

pub fn string_word_counter(s: &str) {
    let mut word_counter_map = MyHash::new();
    let word_vec: Vec<&str> = s.split(" ").collect();

    for i in word_vec {
        if i.len() == 0 {
            continue;
        }
        let string_upc = &i.to_uppercase();

        if word_counter_map.contains_key(string_upc) {
            *word_counter_map.get_mut(string_upc).unwrap() += 1;
        } else {
            word_counter_map.insert(string_upc.to_string(), 1);
        }
    }

    let mut v = Vec::from_iter(word_counter_map);
    v.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    println!("\nTop 5 most used words in the text of len = {} chars.\n", s.len());

    for i in 0..5 {
        let value = v.get(i).unwrap();
        print!("{} => {:?}\n", value.0, value.1);
    }
}

pub fn hashmap_test() {
    let mut shapes = HashMap::new();
    shapes.insert("a", 1234);

    println!("{:?}", shapes);

    let a_val = match shapes.get("a") {
        Some(x) => x,
        None => &0
    };

    let b_val = match shapes.get("b") {
        Some(x) => x,
        None => &0
    };

    println!("a = {:?} b = {}", a_val, b_val);

    let bcontains = shapes.contains_key("a");

    if bcontains {
        println!("{:?} -> {}", "a", shapes.get("a").unwrap());
    }

    let keys = shapes.keys();
    println!("keys = {:?}", keys);

    let values = shapes.values();
    println!("values = {:?}", values);

    println!("Directly replace the value for key .. ");

    let key = "a";
    shapes.insert(key.into(), 5);

    println!("shapes = {:?}", shapes);
    println!("Check if entry does not exist and set value ..");

    shapes.entry("circle".into()).or_insert(123);
    println!("shapes = {:?}", shapes);
    shapes.entry("circle".into()).or_insert(234);
    println!("Tried to replace circle when it existed - shapes = {:?}", shapes);

    let contents = fs::read_to_string("rust-practice/src/test_data/asyoulik.txt")
        .expect("Something went wrong reading the file");

    string_word_counter(&contents);
}
