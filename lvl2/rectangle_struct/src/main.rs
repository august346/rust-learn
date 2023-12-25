#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    left_top: Point,
    right_bottom: Point
}

fn main() {
    let lt_point = Point { x: 1.0, y: 2.5 };
    let rb_point = Point { x: 10.5, y: 0.2 };

    let rectangle = Rectangle { left_top: lt_point, right_bottom: rb_point };

    println!("{:?}", rectangle);
}
