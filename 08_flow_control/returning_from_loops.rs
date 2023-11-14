fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;  // return value after break
        }
    }; // return loops need semi colon

    assert_eq!(result, 20);
}