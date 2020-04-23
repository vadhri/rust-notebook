#![allow(dead_code)]

trait Sum<T> {
    fn sum(&self) -> i32;
}

impl Sum<i32> for Vec<i32> {
    fn sum(&self) -> i32 {
        let mut results: i32 = 0;

        for item in self {
            results += item;
        }
        results
    }
}

pub fn trait_extend_types_test() {
    let myvector: Vec<i32> = vec![1,2,3,4,5];
    println!("trait_extend_types_test -> {:?}", myvector.sum());
}
