fn eat_box_i32(boxed_i32: Box<i32>) {
    println!("Destroying box that contains {}", boxed_i32);
}

fn borrow_i32(borrowed_i32: &i32) {
    println!("This int is: {}", borrowed_i32);
}

fn main() {
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        let mut _ref_to_i32: &i32 = &boxed_i32;

        // Error! Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // FIXME ^ Comment out this line

        borrow_i32(_ref_to_i32);
        // FIXME ^ Comment out this line
    }

    eat_box_i32(boxed_i32);
}