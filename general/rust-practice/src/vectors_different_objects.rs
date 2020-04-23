#![allow(dead_code)]

#[derive(Debug)]
pub struct Triangle {
    area: f32
}

#[derive(Debug)]
pub struct Square {
    area: f32
}

trait Polygon {
    fn area(&self) -> f32;
}

impl Polygon for Triangle {
    fn area (&self) -> f32 {
        200 as f32
    }
}

impl Polygon for Square {
    fn area (&self) -> f32 {
        400 as f32
    }
}

#[derive(Debug)]
enum Shape {
    Triangle (Triangle),
    Square (Square)
}

pub fn vectors_enum_objects_test () {
    let mut shapes = Vec::new();

    shapes.push(Shape::Triangle(Triangle {
        area: 10 as f32
    }));
    shapes.push(Shape::Square(Square {
        area: 20 as f32
    }));

    println!("Shapes -> {:?}", shapes);

    let mut polygons: Vec<Box<dyn Polygon>> = Vec::new();
    polygons.push(Box::new(Triangle { area: 200 as f32 } ));
    polygons.push(Box::new(Square { area: 200 as f32 } ));

    for x in polygons.iter() {
        println!("{:?}", x.area());
    }
}
