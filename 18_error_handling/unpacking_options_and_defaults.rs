#[derive(Debug)]
enum Fruit {
    Apple,
    Orange,
    Banana,
    Kiwi,
    Lemon
}

fn main() {
    println!("-- or --");
    let apple = Some(Fruit::Apple);
    let organge = Some(Fruit::Orange);
    let no_fruit: Option<Fruit> = None;

    let first_fruit = no_fruit.or(organge).or(apple);
    println!("{:?}", first_fruit);
    // apple has been moved so we can no longer use it;
    // println!("{:?}", apple);

    println!("-- or_else --");
    let get_kiwi_fallback = || {
        println!("Providing kiwi as fallback");
        Some(Fruit::Kiwi)
    };

    let get_lemon_fallback = || {
        println!("Providing lemon as fallback");
        Some(Fruit::Lemon)
    };

    let no_fruit: Option<Fruit> = None;
    let first_fruit = no_fruit
        .or_else(get_kiwi_fallback)
        .or_else(get_lemon_fallback);
    // This is Some(Kiwi) and was evaluated lazily meaning
    // get_lemon_fallback was not ran.
    println!("{:?}", first_fruit);

    println!("-- get_or_insert_with --");
    let get_lemon_fallback = || {
        println!("Providing lemon as fallback");
        Fruit::Lemon
    };
    let mut my_fruit: Option<Fruit> = None;
    let first_fruit = my_fruit
        .get_or_insert_with(get_lemon_fallback);
    println!("{:?}", first_fruit); // Lemon
    println!("{:?}", my_fruit); // Some(Lemon);

    let mut my_fruit = Some(Fruit::Apple);
    let should_be_apple = my_fruit.get_or_insert_with(get_lemon_fallback);
    println!("{:?}", should_be_apple);  // Apple
    println!("{:?}", my_fruit);  // Some apple
}