use std::fmt::Debug;

trait PrintInOption {
    fn print_in_option(self);
}

impl<T> PrintInOption for T where
    Option<T>: Debug {
    fn print_in_option(self) {
        println!("{:?}", Some(self));
    }
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    let vec = vec![1, 2, 3];
    print_type_of(&vec);  // alloc::vec::Vec<i32>
    vec.print_in_option();
}