fn apply<F>(f: F) where
    F: FnOnce() {
    f()
}

fn apply_to_3<F>(f: F) -> i32 where
    // closure takes an i32 and returns i32
    F: Fn(i32) -> i32 {
    f(3)
}

fn main() {
    // Fn: The closure uses the capured variable by rerference (&T)
    // FnMut: The closure uses the captured variable by mutable reference (&mut T)
    // FnOnce: The closure uses the captured value by value (T)

    use std::mem;

    let greeting = "hello";
    let mut farewell = "goodbye".to_owned();

    let diary = || {
        println!("I said {}.", greeting);
        farewell.push_str("!!!");
        println!("Then I screemed {}.", farewell);
        println!("Now I can sleep. zzzz");
        // forcing drop forces farewell to be captured by value. Now requries 'FnOnce'.
        mem::drop(farewell);
    };

    // if we change FnOnce to Fn or FnMut in apply, then there will be a compilation error
    // as `diary` is of type FnOnce
    apply(diary);


    let double = |x| 2 * x;
    println!("3 doubled is {}", apply_to_3(double));
}