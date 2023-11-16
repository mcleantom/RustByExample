struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("> Dropping {}", self.name);
    }
}

fn main() {
    let _a = Droppable { name: "a" };

    {
        let _b = Droppable { name: "b" };

        {
            let _c = Droppable { name: "c" };
            let _d = Droppable { name: "d" };
            println!("Exiting block B")
        }
        println!("Just exited block B");
        println!("Exiting block A");
    }
    println!("Exited block A");

    drop(_a);

    // The drop order is D, C, B, A

    println!("End of main function");
}