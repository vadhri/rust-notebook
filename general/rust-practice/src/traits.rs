#![allow(dead_code)]

trait Shape {
    fn area(&mut self) -> f64;
    fn perimeter(&mut self) -> f64;
}

#[derive(Debug, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, PartialEq)]
struct Square {
    l: Point,
    r: Point,
    u: Point,
    d: Point
}

impl Square {
    fn new(l: Point, r: Point, u: Point, d: Point) -> Self {
        Square {
            l: l,
            r: r,
            u: u,
            d: d
        }
    }
}

impl Shape for Square {
    fn area(&mut self) -> f64 {
        let sline = (((self.r.y - self.l.y).pow(2) - (self.r.x - self.l.x).pow(2)) as f64).sqrt();
        sline * sline
    }

    fn perimeter(&mut self) -> f64 {
        let sline = (((self.r.y - self.l.y).pow(2) - (self.r.x - self.l.x).pow(2)) as f64).sqrt();
        (4 as f64) * sline
    }
}

pub fn traits_test () {
    let l = Point{ x: 0, y: 0 };
    let r = Point{ x: 0, y: 10 };
    let u = Point{ x: 10, y: 10 };
    let d = Point{ x: 10, y: 0 };

    let l1 = Point{ x: 0, y: 0 };
    let r1 = Point{ x: 0, y: 10 };
    let u1 = Point{ x: 10, y: 10 };
    let d1 = Point{ x: 10, y: 0 };

    let mut sq = Square::new(l, r, u, d);
    let sq1 = Square::new(l1, r1, u1, d1);

    assert_eq!(sq, sq1);

    println!("Print ->  \nSQ {:?}\nSQ1 {:?}", sq, sq1);
    println!("Area -> {:?} Perimeter -> {:?}", sq.area(), sq.perimeter());
}
