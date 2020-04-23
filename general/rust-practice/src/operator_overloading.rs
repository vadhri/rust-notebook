#![allow(dead_code)]

use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq)]
struct Inventory {
    pens: i32,
    pencils: i32,
    postit: i32,
    books: i32
}

impl Inventory {
    pub fn new(pens: i32, pencils: i32, postit: i32, books: i32) -> Self {
        Inventory {
            pens: pens,
            pencils: pencils,
            postit: postit,
            books: books
        }
    }
}

impl Add for Inventory {
    type Output = Inventory;

    fn add(self, rhs: Self) -> Self::Output {
        let new_pens = self.pens + rhs.pens;
        let new_pencils = self.pencils + rhs.pencils;
        let new_postit = self.postit + rhs.postit;
        let new_books = self.books + rhs.books;

        Inventory::new(new_pens, new_pencils, new_postit, new_books)
    }
}

impl AddAssign for Inventory {
    fn add_assign(&mut self, rhs: Self) {
        self.pens += rhs.pens;
        self.pencils += rhs.pencils;
        self.postit += rhs.postit;
        self.books += rhs.books;
    }
}

pub fn operator_overloading_test() {
    let i1 = Inventory::new(1,2,3,4);
    let i2 = Inventory::new(10, 11, 12, 13);
    let i3 = Inventory::new(10,20,30,40);
    let i4 = Inventory::new(10, 11, 12, 13);

    println!(" i1.add(i2) -->{:?}", i1.add(i2));
    println!(" i1 + i2 --> {:?}", i3 + i4);

    let mut i5 = Inventory::new(100,200,300,400);
    let i6 = Inventory::new(10, 11, 12, 13);

    i5 += i6;

    println!(" i5 += i6 --> {:?}", i5 );
    println!(" compare with PartialEq derive --> {:?}", i5 == Inventory::new(1,2,3,4) );

}
