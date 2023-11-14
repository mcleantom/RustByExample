 use std::mem;

 fn analyze_slice(slice: &[i32]) {
    println!("First element of slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
 }

 fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    let ys: [i32; 500] = [0; 500];

    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    analyze_slice(&xs);
    analyze_slice(&ys[2 .. 4]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{i}: {xval}"),
            None => println!("Out of range")
        }
    }
 }