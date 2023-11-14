// Rust infers a lifetime that is as short as possible
// The two references are then coerced to that lifetime
fn multiply<'a>(first: &'a i32, second: &'a i32) -> i32 {
    first * second
}

// `<'a: 'b, 'b>` reads as a lifetime 'a that is at least as long as 'b
// Here, we take in an &'a i32 and return an &'b i32 as a result of the coercion
fn choose_first<'a: 'b, 'b>(first: &'a i32, _: &'b i32) -> &'b i32 {
    first
}

fn main() {
    let first = 2;
    let third = 3;
    
    {
        let second = 3;
        println!("The first product is {}", multiply(&first, &second));
        println!("{} is the first", choose_first(&first, &second));
        println!("{} is the first", choose_first(&second, &first));
    }
}