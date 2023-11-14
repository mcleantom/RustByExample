mod my;

fn function() {
    println!("called function()");
}

fn main() {
    function();
    my::function();
    my::indirect_access();
    my::nested::function();
    // my::private_function();
    // my::inaccessible::public_function();
}