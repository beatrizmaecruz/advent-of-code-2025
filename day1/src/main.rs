use std::env;
use std::fs;

// PART ONE
// fn main() {
//     // First process the file contents to become a usable vector.
//     let args: Vec<String> = env::args().collect();

//     // Vec<String> is a iterable data type where each element is a line from the input file.
//     let rotation_arr: Vec<String> = process_file(args);

//     let mut current_dial: i32 = 50; // Dial starts at 50. Note we use u8 as dial can be 0 - 99 and u8's range is 0 - 255.
//     let mut zero_counts: u32 = 0;

//     // Process each rotation.
//     for rotation in rotation_arr {
//         current_dial = rotate(&rotation, current_dial);

//         if (current_dial == 0) {
//             zero_counts += 1;
//         }
//         // Afterwards, we need to process every dial and get the result back.
//         // If the result is the dial being 0, we need need to increment zero_counts.
//     }

//     println!("{zero_counts}");
// }

// fn process_file(args: Vec<String>) -> Vec<String> {
//     // let query = &args[1];
//     let file_path = &args[2];
//     let contents = fs::read_to_string(file_path).expect("Should have been able to read the file."); // Turns file content into massive string that is OWNED by contents.
//     // Want to turn contents into a newline split usable array.

//     let lines_vec: Vec<String> = contents.lines().map(String::from).collect();

//     return lines_vec;
// }

// fn rotate(instruction: &String, current_dial: i32) -> i32 {
//     let is_right: bool = &instruction[..1] == "R"; // Getting each rotation
//     let distance: u16 = instruction[1..].parse().expect("Not a valid number!"); // Getting each distance.

//     // Unit tests that the wrapping is correct (0 + L1 = 99 and 99 + R1 = 0)
//     let right_overflow: i32 = 100;
//     let left_overflow: i32 = -1;
//     let modulus: i32 = 100;
//     assert_eq!(right_overflow.rem_euclid(modulus), 0); // Right max.
//     assert_eq!(left_overflow.rem_euclid(modulus), 99); // Left max.

//     // Apply this to actual dial.
//     let mut new_dial: i32;
//     if is_right {
//         new_dial = (current_dial as i32 + distance as i32).rem_euclid(modulus as i32);
//     } else {
//         new_dial = (current_dial as i32 - distance as i32).rem_euclid(modulus as i32);
//     }

//     return new_dial;
// }

// PART TWO
fn main() {
    // First process the file contents to become a usable vector.
    let args: Vec<String> = env::args().collect();

    // Vec<String> is a iterable data type where each element is a line from the input file.
    let rotation_arr: Vec<String> = process_file(args);

    let mut current_dial: i32 = 0; // Dial starts at 50. Note we use u8 as dial can be 0 - 99 and u8's range is 0 - 255.
    let mut zero_counts: u32 = 0;
    let mut zero_passes: i32;

    // Process each rotation.
    for rotation in rotation_arr {
        (current_dial, zero_passes) = rotate(&rotation, current_dial);

        if current_dial == 0 {
            zero_counts += 1;
        }

        zero_counts += zero_passes as u32;
        // Afterwards, we need to process every dial and get the result back.
        // If the result is the dial being 0, we need need to increment zero_counts.
    }

    println!("{zero_counts}");
}

fn process_file(args: Vec<String>) -> Vec<String> {
    // let query = &args[1];
    let file_path = &args[2];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file."); // Turns file content into massive string that is OWNED by contents.
    // Want to turn contents into a newline split usable array.

    let lines_vec: Vec<String> = contents.lines().map(String::from).collect();

    return lines_vec;
}

fn rotate(instruction: &String, current_dial: i32) -> (i32, i32) {
    let is_right: bool = &instruction[..1] == "R"; // Getting each rotation
    let distance: u16 = instruction[1..].parse().expect("Not a valid number!"); // Getting each distance.

    // Unit tests that the wrapping is correct (0 + L1 = 99 and 99 + R1 = 0)
    let right_overflow: i32 = 100;
    let left_overflow: i32 = -1;
    let modulus: i32 = 100;
    assert_eq!(right_overflow.rem_euclid(modulus), 0); // Right max.
    assert_eq!(left_overflow.rem_euclid(modulus), 99); // Left max.

    // Unit tests checking that we are calculating the correct amount of 0s being passed.
    assert_eq!(929_i32.rem_euclid(modulus), 29); // Same as saying 99 + R830. You'll land at 29.
    assert_eq!(929_i32.div_euclid(modulus), 9);

    // Apply this to actual dial.
    let new_dial: i32;
    let mut zero_passes: i32;

    if is_right {
        new_dial = (current_dial as i32 + distance as i32).rem_euclid(modulus);
        zero_passes = (current_dial as i32 + distance as i32).div_euclid(modulus);
    } else {
        new_dial = (current_dial as i32 - distance as i32).rem_euclid(modulus);
        zero_passes = (current_dial as i32 - distance as i32)
            .div_euclid(modulus)
            .abs();
    }

    if new_dial == 0 && !is_right {
        zero_passes -= 1
    }

    // assert_eq!((current_dial as i32 + distance as i32).div_euclid(modulus), 1);

    println!("Moving {instruction} from {current_dial} to {new_dial}, with {zero_passes} passes over 0.");

    return (new_dial, zero_passes);
}
