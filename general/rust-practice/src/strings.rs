#![allow(dead_code)]

pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}

pub fn strings_test() {
    let s: &'static str = "This is a test string";

    println!("{}", s);

    let s1: &'static str = "Lorem Ipsum is simply dummy text of the printing and typesetting industry.";

    let mut s1_chars:Vec<char> = s1.chars().collect();

    println!("chars -> {:?}", s1_chars);
    s1_chars.sort();
    s1_chars.dedup();
    println!("Unique chars -> {:?}", s1_chars);

    // reverse strings
    let mut words:Vec<&str> = s1.split(" ").collect();
    words.sort();
    words.dedup();

    println!("{:?}", words);

    // slicing
    println!("substring 0-9 {:?}", &s1[0..9]);

    // replace sub-strings_test
    let s_replaced = s.replace("test", "actual");

    println!("Replaced string : {}", s_replaced);

    let s_int: i32 = "20".parse().unwrap();

    println!("convert string to number -> {}", s_int);

    let number = 20;
    let s_string = number.to_string();

    println!("convert number to string -> {} ", s_string);

    println!("concatenate strings -> {} ", "Added text !! ".to_string() + s);

    println!("Convert string to bytes -> {:?}", s.as_bytes());

    println!("{:?}", reverse("Testing"));

}
