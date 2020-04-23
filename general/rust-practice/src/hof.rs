#![allow(dead_code)]

pub fn square_of_sum(n: u32) -> u32 {
    (0..=n).fold(0, |sum, x| sum + x).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    (0..=n).map(|x| x * x).fold(0, |sum, x| sum + x)
}

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut mult = Vec::new();

    for num in 1..limit {
        let multiples = factors.iter().map(|f| {
            if *f == 0 {
                0
            } else {
            match num % f {
                0 => 1,
                _ => 0
            }}
        }).fold(0, |sum, x| sum + x);

        if multiples > 0 {
            mult.push(num);
        }
    }
    mult.iter().fold(0, |sum, x| sum + x)
}

pub fn factors(n: u64) -> Vec<u64> {
    let mut n_clone = n;
    let mut vec = Vec::new();
    let mut n_clone_orig = n_clone;

    for num_2 in 2..=2 {
        println!("num = {:?}", num_2);
        while n_clone % num_2 == 0 && n_clone != 1 {
            n_clone = n_clone / num_2;
            vec.push(num_2);
        }
        if n_clone_orig > n_clone {
            n_clone_orig = n_clone;
        }
    }

    for num in (3..=n).step_by(2) {
        println!("num = {:?} {}", num, n_clone);
        while n_clone % num == 0 && n_clone != 1 {
            n_clone = n_clone / num;
            vec.push(num);
        }
        if n_clone_orig > n_clone {
            n_clone_orig = n_clone;
        }

        if n_clone == 1  {
            break;
        }
    }
    vec
}

pub fn is_armstrong_number(num: u32) -> bool {
    let digits: Vec<_> = num.to_string().chars().map(|d| d.to_digit(10).unwrap()).collect();
    let number_of_digits = digits.len() as u32;

    digits.iter().map(|x| x.pow(number_of_digits)).fold(0, |sum, x| sum + x) == num
}

pub fn hof_test() {
    println!("\n Higher order functions test \n");
    let _result:u32 = square_of_sum(5) - sum_of_squares(5);

    println!("sum = {:?} {} {}", square_of_sum(5), sum_of_squares(5), sum_of_multiples(20, &vec![3,5]));

    println!("factors of 64 {:?}", factors(93819012551));

    println!("154 = {:?}", is_armstrong_number(154));
}
