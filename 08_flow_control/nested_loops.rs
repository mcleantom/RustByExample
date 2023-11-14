#![allow(unreachable_code, unused_labels)]

fn main() {
    'outer: loop {
        println!("Entered the outer loop");
        'inner: loop {
            println!("Entered the innter loop");
            break 'outer;
        }
        println!("This will never be reached");
    }
    println!("Exited outer looop");
}