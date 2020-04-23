#![allow(dead_code)]

use std::sync::{Arc, Mutex};
use std::{thread, time};

#[derive(Debug)]
pub struct BankAccount {
    account_no: String,
    money: Arc<Mutex<f64>>
}

impl BankAccount {
    pub fn withdrawl(&mut self, amount: f64) -> f64 {
        let mut balance = self.money.lock().unwrap();
        *balance -= amount;
        // println!("Acc no : {} -> {:?} withdrawn, left {}", self.account_no, amount, *balance);

        *balance
    }

    pub fn deposit(&mut self, amount: f64) -> f64 {
        let mut balance = self.money.lock().unwrap();
        *balance += amount;
        // println!("Acc no : {} -> {:?} deposited, left {}", self.account_no, amount, *balance);

        *balance
    }
}

#[derive(Debug)]
struct Person {
    name: String,
    withdrawl_limit: f64,
    deposit_limit: f64
}

pub fn shopping_arc_test() {
    let mut ba = BankAccount {
        account_no: "ICICI".to_string(),
        money: Arc::new(Mutex::new(10000 as f64))
    };

    let person = Person {
        name: "Adult 1".to_string(),
        withdrawl_limit: 1910 as f64,
        deposit_limit: 1200 as f64,
    };

    let t1 = thread::spawn(move || {
        loop {
            let money = ba.withdrawl(person.withdrawl_limit);
            thread::sleep(time::Duration::from_millis(100));

            if money <= 0.0 {
                println!("{:?} -> {} is out of money !", person.name, ba.account_no);
                break;
            }
        }
    });

    let mut ba1 = BankAccount {
        account_no: "YES!".to_string(),
        money: Arc::new(Mutex::new(10000 as f64))
    };

    let person1 = Person {
        name: "Adult 2".to_string(),
        withdrawl_limit: 1257 as f64,
        deposit_limit: 1200 as f64,
    };

    let t2 = thread::spawn(move || {
        loop {
            let money = ba1.withdrawl(person1.withdrawl_limit);
            thread::sleep(time::Duration::from_millis(100));

            if money <= 0.0 {
                println!("{:?} -> {} is out of money !", person1.name, ba1.account_no);
                break;
            }
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    let mut ba2 = BankAccount {
        account_no: "HDFC".to_string(),
        money: Arc::new(Mutex::new(10000 as f64))
    };

    let ba3 = Arc::new(Mutex::new(BankAccount {
        account_no: "SBI".to_string(),
        money: Arc::new(Mutex::new(10000 as f64))
    }));

    let person2 = Arc::new(Person {
        name: "Adult 1".to_string(),
        withdrawl_limit: 786 as f64,
        deposit_limit: 800 as f64
    });

    let person2_clone1 = person2.clone();

    let t3 = thread::spawn(move || {
        loop {
            let money = ba2.withdrawl(person2_clone1.withdrawl_limit);
            thread::sleep(time::Duration::from_millis(100));

            if money <= 0.0 {
                println!("{:?} -> {} is out of money !", person2_clone1.name, ba2.account_no);
                break;
            }
        }
    });

    let person2_clone2 = person2.clone();
    let ba3_clone1 = ba3.clone();

    let t4 = thread::spawn(move || {
        loop {
            {
                let mut ba3_unwrapped = ba3_clone1.lock().unwrap();
                let money = ba3_unwrapped.withdrawl(person2_clone2.withdrawl_limit);
                if money <= 0.0 {
                    println!("T4 {:?} -> {} is out of money !", person2_clone2.name, ba3_unwrapped.account_no);
                    break;
                } else {
                    println!("T4 withdraw -> balance {:?}", money);
                }
            }

            thread::sleep(time::Duration::from_millis(100));
        }
    });

    let person2_clone3 = person2.clone();
    let ba3_clone1 = ba3.clone();

    let t5 = thread::spawn(move || {
        loop {
            {
                let mut ba3_unwrapped = ba3_clone1.lock().unwrap();
                let money = ba3_unwrapped.withdrawl(person2_clone3.withdrawl_limit);
                if money <= 0.0 {
                    println!("{:?} -> {} is out of money !", person2_clone3.name, ba3_unwrapped.account_no);
                    break;
                } else {
                    println!("T5 withdraw -> balance {:?}", money);
                }
            }

            thread::sleep(time::Duration::from_millis(100));
        }
    });

    let person2_clone3 = person2.clone();
    let ba3_clone2 = ba3.clone();

    let t6 = thread::spawn(move || {
        loop {
            {
                let mut ba3_unwrapped = ba3_clone2.lock().unwrap();
                let money = ba3_unwrapped.deposit(person2_clone3.deposit_limit);

                if money >= 0.0 {
                    println!("T6 {:?} -> {} is deposited money -> {}!", person2_clone3.name, ba3_unwrapped.account_no, money);
                } else {
                    break;
                }
            }
            thread::sleep(time::Duration::from_millis(300));
        }
    });

    t3.join().unwrap();
    t4.join().unwrap();
    t5.join().unwrap();
    t6.join().unwrap();
}
