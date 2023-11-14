fn apply<F>(f: F) where F: Fn() {
    f();
}

fn call_me<F: Fn()>(f: F) {
    f();
}

fn function() {
    println!("I am a function!");
}


fn main() {
    let mut x = 7;
    let print = || println!("{}", x);
    apply(print);

    let closure = || println!("I am a closure!");
    call_me(closure);
    call_me(function);
}