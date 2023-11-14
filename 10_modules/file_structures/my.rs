mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called my::function()");
}

fn private_fn() {
    println!("called my::private_fn()");
}

pub fn indirect_access() {
    print!("called my::indirect_access(), that\n> ");
    private_fn();
}