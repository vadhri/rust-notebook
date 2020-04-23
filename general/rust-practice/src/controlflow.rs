pub fn check_body_temperature(input:i32) {
    if input < 98 {
        println!("You must be cold!")
    } else if input < 100 {
        println!("You seem to be doing okay!!")
    } else {
        println!("You might be having a fever!")
    }
}

pub fn while_loop_println(input: i32) {
    let mut i:i32 = 1;
    while i < input {
        println!("i = {}", i);
        i += 1;
    }

    let mut l = 1;

    println!("\n .. loop until 100 ..");

    loop {
        l *= 2;
        println!("{:?}", l);

        if l > 100 {
            break;
        }
    }
}

pub fn for_loop(input: i32) {
    for x in 1..input {
        println!("x = {}", x);
    }
}

pub fn for_loop_range_with_index(input: i32) {
    for (pos, y) in (0 .. input).enumerate() {
        println!("{} is at {}", pos, y);
    }
}

pub fn match_country_code_phone_number(input: i32) {
    let country = match input {
        91 => "India",
        62 => "Indonesia",
        353 => "Ireland",
        001 => "US",
        44 => "UK",
        41 => "Swiss",
        2..=10 => "Unknown",
        _ => "Invalid"
    };

    println!("input = {} country = {}", input, country);
}
