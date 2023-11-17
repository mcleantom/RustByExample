fn give_adult(drink: Option<&str>) {
    match drink {
        Some("lemonade") => println!("lemonade is too sugary"),
        Some(inner) => println!("I like {}", inner),
        None => println!("Cant drink nothing :("),
    }
}

fn drink(drink: Option<&str>) {
    // unwap returns 'panic' when it recieves a None
    let inside = drink.unwrap();
    if inside == "lemonade" { panic!("ahhhh"); }
    println!("I love {}", inside);
}

fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let void = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");

    drink(coffee);
    drink(void);
}