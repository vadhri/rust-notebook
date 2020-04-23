#![allow(dead_code)]

struct OriginPoint {
    x: i32,
    y: i32
}

struct Rectangle {
    origin: OriginPoint,
    width: i32,
    height: i32
}

impl Rectangle {
    pub fn area(&self) -> i32 {
        self.width * self.height
    }

    pub fn permiter_fencing(&self) -> i32 {
        2 * (self.width + self.height)
    }
}

pub fn heap_test() {
    //using stack
    const RECT:Rectangle = Rectangle { origin: OriginPoint { x: 0, y:0 }, width: 200, height: 200 };

    println!("Stack -> Rectangle area = {} ", RECT.area());
    println!("Stack -> Rectangle Perimter fencing = {} ", RECT.permiter_fencing());

    // using heap.

    let rect_heap = Box::new(Rectangle { origin: OriginPoint { x: 0, y:0 }, width: 200, height: 200 });

    println!("Heap -> Rectangle area = {} ", rect_heap.area());
    println!("Heap -> Rectangle Perimter fencing = {} ", rect_heap.permiter_fencing());

}
