#![allow(dead_code)]

use core::convert::TryInto;
use core::convert::TryFrom;

#[derive(Debug)]
struct BillAmount {
    value: f32
}

impl From<i32> for BillAmount {
    fn from(amount: i32) -> Self {
        BillAmount { value: amount as f32 }
    }
}

impl TryFrom<String> for BillAmount {
    type Error = ();

    fn try_from (value: String) -> Result<Self, Self::Error> {
        let number = value.parse::<f32>();

        match number {
            Ok(val) => Ok(BillAmount {
                value: val
            }),
            Err(_reason) => Err(())
        }
    }
}

pub fn conversion_from_into_test() {
    let amount = 12;

    let nbill = BillAmount::from(amount);
    println!("Created with From -> {:?}", nbill);

    let nbill_into: BillAmount = amount.into();
    println!("Created with Into -> {:?}", nbill_into);

    let amount_string = "200";

    let result: Result<BillAmount, ()> = BillAmount::try_from(amount_string.to_string());
    println!("nBill string (try from) -> {:?}", result);

    let result: Result<BillAmount, ()> = amount_string.to_string().try_into();
    println!("nBill string (try into) -> {:?}", result);
}
