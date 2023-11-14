/*

Iterator::any is a function which when passed an iterator, will return true if any element satisfies the predicate.
Otherwise false. Its signature:

pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `any` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn any<F>(&mut self, f: F) -> bool where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `Self::Item` states it takes
        // arguments to the closure by value.
        F: FnMut(Self::Item) -> bool;
}

*/

/*

Iterator::find is a function which iterates over an iterator and searches for the first value which satisfies some condition. If none of the values satisfy the condition, it returns None.
Its signature:

pub trait Iterator {
    // The type being iterated over.
    type Item;

    // `find` takes `&mut self` meaning the caller may be borrowed
    // and modified, but not consumed.
    fn find<P>(&mut self, predicate: P) -> Option<Self::Item> where
        // `FnMut` meaning any captured variable may at most be
        // modified, not consumed. `&Self::Item` states it takes
        // arguments to the closure by reference.
        P: FnMut(&Self::Item) -> bool;
}

*/

fn main() {
    // ANY
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    // iter() yields &i32. Destructure to i32 using |&x|
    println!("2 in vec1: {}", vec1.iter().any(|&x| x == 2));
    // into_iter() yields i32
    println!("2 in vec2: {}", vec2.into_iter().any(|x| x == 2));

    // // iter() only borrows vec1 and its elements, so they can be used again
    println!("vec1 len: {}", vec1.len());
    println!("First element: {}", vec1[0]);

    // // into_iter() moves vec2 and its elements, so they cant be used again
    // println!("vec2 len: {}", vec2.len());
    // println!("First element: {}", vec2[0]);

    let vec2 = vec![4, 5, 6];
    // We can clone vec2 before doing into_iter
    println!("2 in vec2: {}", vec2.clone().into_iter().any(|x| x == 2));
    println!("vec2 len: {}", vec2.len());
    println!("First element: {}", vec2[0]);

    // FIND
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];

    let mut iter = vec1.iter();
    let mut into_iter = vec2.into_iter();

    // iter() for vecs yields &i32 so we want to refernece one of its items
    // so destructure &&i32 to i32
    println!("Find 2 in vec1: {:?}", iter.find(|&&x| x == 2));

    // into_ter() for vecs yields &i32 and we want to reference one of its items
    // so destructure &i32 to i32
    println!("Find 2 in vec2: {:?}", into_iter.find(|&x| x == 2));

    let vec3 = vec![7, 8, 9];
    // you can get the index using .position
    let index = vec3.iter().position(|&x| x == 9);
    match index {
        Some(i) => println!("Index: {:?}, Value: {:?}", i, vec3[i]),
        None => println!("No index found"),
    }
}