use deeply::nested::function as other_function;

fn function() {
    println!("called function");
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("Called deeply nested function");
        }
    }
}


fn main() {
    other_function();
    {
        use crate::deeply::nested::function;
        function();  // calls deeply nested funciton instead
    }
    function();
}