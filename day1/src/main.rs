use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file: String = process_file(args);
    println!("{}", file);

    let mut _current_dial: u8 = 50; // Dial starts at 50. Note we use u8 as dial can be 0 - 99 and u8's range is 0 - 255.
    let mut _zero_counts: u32 = 0;

    // println!("With text:\n{contents}");

    // First process the file contents to become a usable array.
    // Each element in the array is a tuple of (char, u16) which is 'R'/'L' then the number of dial shifts.

    // Afterwards, we need to process every dial and get the result back.
    // If the result is the dial being 0, we need need to increment zero_counts.

    // O + L1 = 99
    // 99 + R1 = 0. In this case, we need to mod it by 100.
    // Check out .rem_euclid() for Rust.
}

fn process_file(args: Vec<String>) -> String {
    // let query = &args[1];
    let file_path = &args[2];

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    contents
}
