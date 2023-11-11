#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

struct Unit;

struct Pair(i32, f32);

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Debug)]
struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rect: Rectangle) -> f32 {
    /*
    Calculate the area of a rectangle.
    */
    let Rectangle { 
        top_left: Point { x: x1, y: y1 },
        bottom_right: Point { x: x2, y: y2 }
    } = rect;
    return (x2 - x1).abs() * (y2 - y1).abs();
}

fn square(tl: Point, width: f32) -> Rectangle {
    /*
    Create a square from a top left point and a width.
    */
    
    Rectangle {
        top_left: Point { x: tl.x, y: tl.y },
        bottom_right: Point { x: tl.x + width, y: tl.y - width }
    }
}

fn main() {
    let name = String::from("Tom");
    let age = 26;
    let tom = Person { name, age };

    println!("{tom:?}");

    let point = Point { x: 10.3, y: 0.4 };
    let bottom_right = Point { x: 5.2, ..point };

    println!("{bottom_right:?}");

    let Point { x: top_edge, y: left_edge } = point;

    let _rectangle = Rectangle {
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    println!("{_rectangle:?}");

    let _unit = Unit;
    let pair = Pair(1, 0.1);
    let Pair(integer, decimal) = pair;
    println!("pair contains {:?} and {:?}", integer, decimal);

    let area = rect_area(_rectangle);
    println!("area is {:?}", area);

    let sq = square(Point { x: 0.0, y: 0.0 }, 5.0);
    println!("sq is {:?}", sq);
}