mod my_mod{
    fn private_function() {
        println!("called private fn");
    }

    pub fn function() {
        println!("Called pub fn");
    }

    pub fn indirect() {
        println!("Called indirect");
        private_function();
        function();
    }

    pub mod nested {
        pub fn function() {
            println!("Called nested fn");
        }

        pub(super) fn pub_super_fn() {
            println!("Called pub_super_fn");
        }
    }

    mod private_nested {
        #![allow(dead_code)]
        pub fn function() {
            println!("pub fn in private mod");
        }
    }

    pub fn call_nested_super_fn() {
        nested::pub_super_fn();
    }

    pub(crate) fn pub_function_in_crate() {
        println!("This is only public within this crate");
    }
}

fn function() {
    println!("Called fn outisde mod");
}

fn main() {
    function();
    my_mod::function();
    my_mod::indirect();
    my_mod::nested::function();
    my_mod::call_nested_super_fn();
    my_mod::pub_function_in_crate();

    // these are private from this scope::
    // my_mod::private_function();
    // my_mod::nested::pub_super_fn();
    // my_mod::private_nested::function();
}