struct Empty;
struct Null;

trait DoubleDrop<T> {
    fn double_drop(self, _: T);
}

impl<T, U> DoubleDrop<T> for U {
    // This takes ownership of self and T, deallocating both at the end of the function.
    fn double_drop(self, _: T) {
        println!("Dropping type {} and {}", std::any::type_name::<T>(), std::any::type_name::<U>());
    }
}

fn main() {
    let empty = Empty;
    let null = Null;
    
    // Deallocate `empty` and `null`.
    empty.double_drop(null);

    let x = 1;
    let y = "hi".to_string();
    x.double_drop(y);
}