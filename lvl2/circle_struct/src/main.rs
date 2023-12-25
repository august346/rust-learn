#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Circle {
    center: Point,
    radius: f32
}

fn main() {
    let circle = Circle {
        center: Point {x: 5.3, y: 2.0},
        radius: 3.7,
    };


    println!("{:?}", circle);
}
