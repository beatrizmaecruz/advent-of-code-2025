use std::env;
use std::fs;

fn main() {


    // First process the file contents to become a usable vector.
    let args: Vec<String> = env::args().collect();

    // Vec<String> is a iterable data type where each element is a line from the input file.
    let rotation_arr: Vec<String>  = process_file(args);


    let current_dial: u8 = 50; // Dial starts at 50. Note we use u8 as dial can be 0 - 99 and u8's range is 0 - 255.
    let mut _zero_counts: u32 = 0;

    // Process each rotation.
    for rotation in rotation_arr {
        rotate(&rotation, current_dial);
    }



    // Tests that the wrapping is correct (0 + L1 = 99 and 99 + R1 = 0)
    let right_overflow: i32 = 100;
    let left_overflow: i32 = -1;
    let modulus: i32 = 100;
    assert_eq!(right_overflow.rem_euclid(modulus), 0);
    assert_eq!(left_overflow.rem_euclid(modulus), 99);


    // Afterwards, we need to process every dial and get the result back.
    // If the result is the dial being 0, we need need to increment zero_counts.

}

fn process_file(args: Vec<String>) -> Vec<String>{
    // let query = &args[1];
    let file_path = &args[2];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file."); // Turns file content into massive string that is OWNED by contents.
    // Want to turn contents into a newline split usable array.

    let lines_vec: Vec<String> = contents.lines().map(String::from).collect();

    return lines_vec;
}

fn rotate(instruction : &String, current_dial : u8) {
    println!("{}", &instruction[..1]); // Getting each rotation
    println!("{}", &instruction[1..]); // Getting each distance
}