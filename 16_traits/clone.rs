#[derive(Debug, Clone, Copy)]
struct Unit;

#[derive(Clone, Debug)]
struct Pair(Box<i32>, Box<i32>);

fn main() {
    let unit = Unit;

    let copied_unit = unit;

    println!("original: {:?}", unit);
    println!("copied: {:?}", copied_unit);

    let pair = Pair(Box::new(1), Box::new(2));
    println!("Original: {:?}", pair);

    let moved_pair = pair;
    println!("moved: {:?}", moved_pair);

    let cloned_pair = moved_pair.clone();

    drop(moved_pair);

    println!("cloned: {:?}", cloned_pair);
}