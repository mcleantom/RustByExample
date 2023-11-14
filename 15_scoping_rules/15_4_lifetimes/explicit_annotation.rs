/*

foo<'a> means that the lifetime of foo cannot exceed 'a.

*/

fn print_refs<'a, 'b>(x: &'a i32, y: &'b i32) {
    println!("x is {} and y is {}", x, y);
}

fn failed_borrow<'a>() {
    let _x = 12;

    let y: &'a i32 = &_x;

    println!("{}", y);
    println!("{}", _x);
}


fn main() {
    let (four, nine) = (4, 9);

    print_refs(&four, &nine);

    failed_borrow();
}