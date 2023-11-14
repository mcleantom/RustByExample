struct Sheep { naked: bool, pub name: &'static str }

trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }

    fn change_name(&mut self, name: &'static str);
}

impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }

    fn shear(&mut self) {
        if self.is_naked() {
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name());
            self.naked = true;
        }
    }
}

impl Animal for Sheep {
    fn new(name: &'static str) -> Sheep {
        Sheep { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "baaaaah?"
        } else {
            "baaaaaah!"
        }
    }

    fn talk(&self) {
        println!("{} pauses briefly... {}", self.name, self.noise());
    }

    fn change_name(&mut self, name: &'static str) {
        println!("{} will henceforth be known as {}", self.name, name);
        self.name = name;
    }
}


fn main() {
    let mut dolly: Sheep = Animal::new("Dolly");
    dolly.talk();
    dolly.shear();
    dolly.talk();
    dolly.change_name("Dolly Jr.");
    dolly.talk();

    let steve: Sheep = Animal::new("Steve");
    steve.talk();
    // not possible as steve is immutable
    // steve.change_name("John");
    // steve.talk();
}