use std::fmt::Debug;


trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

trait HasPerimeter {
    fn perimeter(&self) -> f64;
}

impl HasPerimeter for Rectangle {
    fn perimeter(&self) -> f64 { 2.0 * (self.length + self.height) }
}

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn perimeter<T: HasPerimeter>(t: &T) -> f64 { t.perimeter() }

fn area_and_perimeter<T: HasArea + HasPerimeter>(t: &T) -> (f64, f64) { (t.area(), t.perimeter()) }

fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

struct Cardinal;
struct BlueJay;
struct Turkey;

trait Red {}
trait Blue {}

impl Red for Cardinal {}
impl Blue for BlueJay {}

fn red<T: Red>(_: &T) -> &'static str { "red" }
fn blue<T: Blue>(_: &T) -> &'static str { "blue" }

fn main() {
    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    // print_debug(&triangle);
    
    println!("Area of rectangle: {}", area(&rectangle));
    println!("Perimeter of rectangle: {}", perimeter(&rectangle));
    // println!("Area of triangle: {}", area(&triangle));
    // println!("Perimeter of triangle: {}", perimeter(&triangle));
    println!("Area and perimeter of rectangle: {:?}", area_and_perimeter(&rectangle));
    // println!("Area and perimeter of triangle: {:?}", area_and_perimeter(&triangle));

    let cardinal = Cardinal;
    let blue_jay = BlueJay;
    let _turkey   = Turkey;

    println!("A cardinal is {}", red(&cardinal));
    println!("A blue jay is {}", blue(&blue_jay));
    // println!("A turkey is {}", red(&_turkey));
}