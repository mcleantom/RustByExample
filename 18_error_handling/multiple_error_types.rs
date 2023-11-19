use std::num::ParseIntError;

fn double_first(vec: Vec<&str>) -> Result<Option<i32>, ParseIntError> {
    let opt = vec.first().map(|first| {
        first.parse::<i32>().map(|n| 2 * n)
    });

    opt.map_or(Ok(None), |r| r.map(Some))
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

