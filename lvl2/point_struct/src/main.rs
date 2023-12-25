#![allow(dead_code)]

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

fn main() {
    let point = Point { x: 1.0, y: 2.5 };
    println!("{:?}", point);
}
