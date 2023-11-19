fn main() {
    let strings = vec!["tofu", "93", "18"];
    let numbers: Vec<_> = strings
        .into_iter()
        .map(|s| s.parse::<i32())
        .collect();
    println!("Results: {:?}", numbers);

    let numbers = strings
        .into_iter()
        .filter_map(|s| s.parse<i32>().ok())
        .collect();
    println!("Results: {:?}", numbers);

    let mut errors = vec![];
    let numbers = strings
        .into_iter()
        .map(|s| s.parse::<u8>())
        .filter_map(|r| r.map_err(|e| errors.push(e)).ok())
        .collect();
    println!("Numbers: {:?}", numbers);
    println!("Errors: {:?}", errors); 
}