use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Option<Result<i32, ParseIntError>> {
    vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    })
}

macro_rules! print {
    ($x:expr) => {
        println!("{:?}", $x);
    }
}

fn main() {
    let numbers = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["one", "two", "three"];
    print!(double_first(numbers));
    print!(double_first(empty));
    print!(double_first(strings));
}

