use std::convert::From;
use std::convert::Into;

#[derive(Debug)]
struct Number {
    value: i32,
}

impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// We get this for free with 'From'
// impl Into<Number> for i32 {
//     fn into(self) -> Number {
//         Number { value: self }
//     }
// }

fn main() {
    let num = Number::from(30);
    println!("Number is {num:?}");
    let int: i32 = 5;
    let num_2: Number = int.into();
    println!("Number is {num_2:?}");
}