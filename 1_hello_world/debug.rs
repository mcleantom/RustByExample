#[derive(Debug)]
struct Structure(i32);

#[derive(Debug)]
struct Deep(Structure);

#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main() {
    println!("{:?} months in a year.", 12);
    println!("Now {:?} will print!", Structure(3));
    println!("Now {:?} will print!", Deep(Structure(7)));
    let x: Structure = Structure(3);
    println!("{x:?}");

    // pretty print with {:#?}
    let name = "Tom";
    let age = 26;
    let tom = Person { name, age };
    println!("{tom:#?}");
}