fn drink(beverage: &str) {
    if beverage == "lemonade" {
        if cfg!(panic="abort") {
            println!("aborrrrrrrtt");
        }
        else {
            println!("Spit it out");
        }
    }
    else {
        println!("yay {}", beverage);
    }
}

#[cfg(panic="unwind")]
fn ah() {
    println!("Spit it out");
}

#[cfg(not(panic="unwind"))]
fn ah() {
    println!("aborrrrrrrrrrrt");
}

fn drink2(beverage:&str) {
    if beverage == "lemonade" { ah(); }
    else {
        println!("yay {}", beverage);
    }
}

fn main() {
    drink("water");
    drink("lemonade");
    drink2("water");
    drink2("lemonade");
}