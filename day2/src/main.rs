use std::env;
use std::fs;

fn main() {
    // PART 1: File proccessing.
    // Process file into long string.

    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];
    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file."); // Turns file content into massive string that is OWNED by contents.

    // Separate by comma (into Vec<String> iterable) into ID ranges.
    let ranges: Vec<&str> = contents.trim().split(',').collect();
    // Here contents still owns the original string. But ranges owns the Vec itself, a: "a contiguous, resizable array type that stores a collection of values of the same type on the heap"
    // In this case, it is an &str types in the Vector array where each element is a fat pointer towards their slice in the contents String.

    // SELF NOTE ON RUST
    // contents NEEDS to be of type &str due to how .collect and Rust works.
    // Rust aims for a 'zero-cost' abstraction: don't pay for features you don't use!
    // "If collect() accepted owned Strings by value, it would force unnecessary new memory allocations and data copies every time, which is inefficient if the source data is already available as a reference."
    // "By accepting &str, collect() can work with both string literals (which are &'static str) and references to existing Strings, utilizing the existing memory location without immediate re-allocation."

    // Separate each range by dash eg: XX-YY into "XX" and "YY".
    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();

        let start: &str = parts[0];
        let end: &str = parts[1];

        println!("{start} to {end}");
    }

    // PART 2: Processing each ID.
    // Initalize a sum of 0.
    // Iterate through each range, getting numbers from XX to YY.
    // Check if the numbers has repeating patterns. <- will be a function in itself (does_number_have_pattern, using Regex maybe?)
    // If return true, then add the number to sum.
}
