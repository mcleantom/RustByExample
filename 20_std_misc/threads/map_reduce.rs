use std::thread;

fn main() {
    // The data to process
    let data = "86967897737416471853297327050364959
                11861322575564723963297542624962850
                70856234701860851907960690014725639
                38397966707106094172783238747669219
                52380795257888236525459303330302837
                58495327135744041048897885734297812
                69920216438980873548808413720956532
                16278424637452589860345374828574668";

    // Make a vector to hold the child-threads which we will spawn.
    let mut children = vec![];

    let chunked_data = data.split_whitespace();
    
    // Iterate over the data segments.
    for (i, data_segment) in chunked_data.enumerate() {
        println!("data segment {} is \"{}\"", i, data_segment);

        // Process each data segment in a separate thread
        children.push(thread::spawn(move || -> u32 {
            // Calculate the intermediate sum of this segment
            let result = data_segment
                .chars()
                .map(|c| c.to_digit(10).expect("should be a digit"))
                .sum();

            println!("processed segment {}, result={}", i, result);

            result
        }));
    }

    // Collect and combine the intermediate results into a final result
    let final_result = children.into_iter().map(|c| c.join().unwrap()).sum::<u32>();

    println!("Final sum result: {}", final_result);
}
