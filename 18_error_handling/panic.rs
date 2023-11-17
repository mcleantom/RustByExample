fn drink(beverage:&str) {
    if beverage == "lemonade" {
        panic!("Noooooo");
    }
    println!("Yay {}", beverage);
}

fn main() {
    drink("water");
    drink("lemonade");
    drink("still water");
}