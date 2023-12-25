use draw::*;

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32
}

#[derive(Debug)]
struct Rectangle {
    p0: Point,  //lt
    p1: Point   //rb
}

#[derive(Debug)]
struct Circle {
    p0: Point,
    radius: f32,
}

impl From<Rectangle> for Drawing {
    fn from(item: Rectangle) -> Self {
        return Drawing::new()
        // give it a shape
        .with_shape(Shape::Rectangle {
            width: (item.p1.x - item.p0.x) as u32,
            height: (item.p0.y - item.p1.y) as u32,
        })
        // move it around
        .with_xy(item.p0.x, item.p1.y)
        // give it a cool style
        .with_style(Style::stroked(5, Color::black()));
    }
}

impl From<Circle> for Drawing {
    fn from(item: Circle) -> Self {
        return Drawing::new()
        .with_shape(Shape::Circle {
            radius: item.radius as u32
        })
        .with_xy(item.p0.x, item.p0.y)
        .with_style(Style::filled(Color::random()));
    }
}


fn main() {
    // create a canvas to draw on
    let mut canvas = Canvas::new(100, 100);

    // create a new drawing
    let d_rect_0 = Drawing::from(
        Rectangle {
            p0: Point { x: 10.0, y: 90.0},
            p1: Point { x: 90.0, y: 10.0},
        }
    );

    // create a new drawing
    let d_rect_1 = Drawing::from(
        Rectangle {
            p0: Point { x: 20.0, y: 50.0},
            p1: Point { x: 80.0, y: 20.0},
        }
    );

    let circle_0 = Drawing::from(
        Circle {
            p0: Point { x: 30.0, y: 70.0},
            radius: 15.0,
        }
    );

    let circle_1 = Drawing::from(
        Circle {
            p0: Point { x: 70.0, y: 70.0},
            radius: 15.0,
        }
    );

    // add it to the canvas
    canvas.display_list.add(d_rect_0);
    canvas.display_list.add(d_rect_1);
    canvas.display_list.add(circle_0);
    canvas.display_list.add(circle_1);

    // save the canvas as an svg
    render::save(
        &canvas,
        "tests/svg/two_rects_and_two_circles.svg",
        SvgRenderer::new(),
    )
    .expect("Failed to save")
}