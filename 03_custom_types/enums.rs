enum MathOperations {
    Add,
    Subtract
}

impl MathOperations {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let x = MathOperations::Add.run(1, 2);
    let y = MathOperations::Subtract.run(1, 2);
    println!("{x}, {y}");
}