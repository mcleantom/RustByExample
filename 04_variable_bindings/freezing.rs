fn main() {
    let mut _mutable_integer = 7i32;
    {
        let _mutable_integer = _mutable_integer;
        println!("{}", _mutable_integer);
        // _mutable_integer = 50;  // changed from mut to not mut, cant change value
        // println!("{}", _mutable_integer);
    }
    _mutable_integer = 3;
    println!("{}", _mutable_integer);
}