fn main() {
    use std::mem;

    let color = String::from("green");
    let print = || println!("`color`: {}", color);
    print();
    let _reborrow = &color;
    print();
    let _moved = color;
    // On the line above, the color is moved so we cannot call print again
    // print();

    let mut count = 0;
    // we have a mut variable inside, so inc must be mut as well
    let mut inc = || {
        count += 1;
        println!("`count`: {}", count);
    };
    inc();
    // inc mutably borrows count, so an attempt to reborrow will lead to an error
    // let _reborrow = &count;
    inc();

    let movable = Box::new(3);

    let consume = || {
        println!("`movable`: {:?}", movable);
        mem::drop(movable);
    };

    consume();
    // consume can only be called once as mem::drop implements FnOnce
    // consume();

    use std::cell::RefCell;

    let x = RefCell::new(2);
    let mut mutable_borrow = x.borrow_mut();
    *mutable_borrow = 1;
    mem::drop(mutable_borrow);
    let borrow = x.borrow();
    println!("{}", borrow);

    // move before vertical pipes forces closure to take ownership

    let haystack = vec![1, 2, 3];
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // this line does not compile as contains now owns haystack
    // println!("{:?}", haystack);

    let haystack = vec![1, 2, 3];
    let contains = |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // this line does not compile as contains now owns haystack
    println!("{:?}", haystack);
}