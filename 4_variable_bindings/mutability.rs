fn main() {
    let mut mutable_binding = 1;
    println!("{}", mutable_binding);
    mutable_binding += 2;
    println!("{}", mutable_binding);

    let _imutable_binding = 1;
    // _imutable_binding = 2; // illegal
}