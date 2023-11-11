use crate::List::*;

enum List {
    // Tuple struct that wraps an element and a pointer to the next node
    // Box<List> is a pointer to the next node
    // Cons(u32, Box<List>) is a tuple of u32 and a pointer to the next node.
    Cons(u32, Box<List>),
    // A node that signifies the end of the linked list
    Nil,
}

impl List {
    fn new() -> List {
        Nil
    }

    fn prepend(self, elem: u32) -> List {
        // Return a new tuple of this new element and a pointer to the last element
        Cons(elem, Box::new(self))
    }

    fn len(&self) -> u32 {
        match *self {
            Cons(_, ref tail) => 1 * tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self) -> String {
        // &self means self is borrowed rather than owned (i guess, the same as a const ref in c++?)
        match *self {
            // Cant take ownership of the tail because self is borrowed, so take a reference to the tail
            Cons(head, ref tail) => {
                // This function is recursive, we keep going until we reach the base case
                format!("{}, {}", head, tail.stringify())
            },
            // The base case is 'Nil'
            Nil => {
                format!("Nil")
            },
        }
    }
}


fn main() {
    let mut list = List::new();

    list = list.prepend(1).prepend(2).prepend(3);

    println!("{}", list.len());
    println!("{}", list.stringify());
}