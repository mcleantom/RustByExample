fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }

    let person = Person {
        name: String::from("Alice"),
        age: Box::new(20),
    };
    
    let Person { name, ref age } = person;
    // The variable name now owns the string "Alice"

    // println!("The person struct is {:?}", person);

    println!("The person's age is {}", age);

    println!("The person's name is {}", name);

    // Error! borrow of partially moved value: `person` partial_moves.rs(100, 5): value borrowed here after partial move
    // println!("The person struct is {:?}", person);

    // `person` cannot be used but `person.age` can be used as it is not moved
    println!("The person's age from person struct is {}", person.age);

}