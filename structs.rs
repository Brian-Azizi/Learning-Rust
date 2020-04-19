#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

struct Nil;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    let Rectangle { top_left, bottom_right } = rect;
    let Point { x: top_x, y: top_y } = top_left;
    let Point { x: bottom_x, y: bottom_y } = bottom_right;
    let height = top_y - bottom_y;
    let width = bottom_x - top_x;
    height * width
}

fn square(p: Point, w: f32) -> Rectangle {
    let Point { x: left_edge, y: bottom_edge } = p;
    Rectangle { top_left: Point {x: left_edge, y: bottom_edge + w}, bottom_right: Point {x: left_edge + w, y: bottom_edge }}
}

fn main() {
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

    println!("{:?}", peter);

    let point: Point = Point { x: 10.3, y: 0.4 };
    println!("({}, {})", point.x, point.y);

    let bottom_right = Point { x: 5.2, ..point };
    println!("({}, {})", bottom_right.x, bottom_right.y);

    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    let _nil = Nil;

    let pair = Pair(1, 0.1);

    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("{:?}", rect_area(_rectangle));

    println!("{:?}", square(point, 1.0));
}