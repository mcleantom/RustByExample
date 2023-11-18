use std::fmt;

type Result<T> = std::result::Result<T, DoubleError>;

#[derive(Debug, Clone)]
struct EmptyVecError;

#[derive(Debug)]
enum DoubleError {
    ParseIntError,
    EmptyVecError
}

impl fmt::Display for DoubleError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "invalid first item to double")
    }
}

fn double_first(vec: Vec<&str>) -> Result<i32> {
    vec.first()
        .ok_or(DoubleError::EmptyVecError)
        .and_then(|s| {
            s.parse::<i32>()
                .map_err(|_| DoubleError::ParseIntError)
                .map(|i| 2 * i)
        })
}

fn print(result: Result<i32>) {
    match result {
        Ok(n) => println!("The first doubled is {}", n),
        Err(e) => println!("Error: {:?}", e)
    }
}

fn main() {
    let numbers = vec!["1", "2", "3"];
    let empty = vec![];
    let strings = vec!["one", "two", "three"];

    print(double_first(numbers));
    print(double_first(empty));
    print(double_first(strings));
}