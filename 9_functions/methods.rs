use rand::Rng;

struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    fn area(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // self is mutable
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// Pair owns two heap allocated integers
#[derive(Debug)]
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    fn destroy(self) {
        // We take ownership of first and second in the scope of this method
        let Pair(first, second) = self;
        println!("Destroying Pair({}, {})", first, second);
    }

    fn add(&self) -> i32 {
        let Pair(ref first, ref second) = *self;
        **first + **second
    }
}

fn main() {
    let rectangle = Rectangle {
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // the rectangle is implicitly passed to Rectangle::perimeter(&self)
    println!("Rectangle perimeter {}", rectangle.perimeter());
    println!("Rectangle area {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // rectangle is not mut so we cant do this
    // rectangle.translate(1.0, 1.0);

    // square is mut so we can do this
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("Pair adds to {}", pair.add());

    // 50% of the time, destroy the pair
    if rand::thread_rng().gen::<bool>() {
        pair.destroy();
    }

    // can no longer do this as we dont own the pair anymore
    // println!("Pair adds to {}", pair.add());

    // however we can do....
    let pair = Pair(Box::new(1), Box::new(2));
    println!("Pair adds to {}", pair.add());
    if rand::thread_rng().gen::<bool>() {
        pair.destroy();
    } else {
        println!("Pair add to {}", pair.add());
    }

}