use fancy_regex::Regex;
use std::env;
use std::fs;

fn main() {
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

    // Initalize a sum of 0 and regex (and only refer to it so re always own the Regex.)
    let mut sum: u64 = 0;
    let re = Regex::new(r"(?!\1)^(\d+)\1(?!\1)$").unwrap();

    for range in ranges {
        let parts: Vec<&str> = range.split("-").collect();

        // Separate each range by dash eg: XX-YY into "XX" and "YY".
        let start_str: &str = parts[0];
        let end_str: &str = parts[1];
        // Convert into workable integers.
        let start: u64 = start_str.parse().expect("Expected number.");
        let end: u64 = end_str.parse().expect("Expected number.");

        // Find matches
        let matches = find_pattern_in_ranges(start, end, &re);
        for pattern in matches {
            sum += pattern;
        }
    }
    println!("{sum}");
}

fn find_pattern_in_ranges(start: u64, end: u64, re: &Regex) -> Vec<u64> {

    let mut matches: Vec<u64> = Vec::new();
    // Iterate from start to end.
    for num in start..=end {
        // Check if the numbers has repeating patterns. <- will be a function in itself (does_number_have_pattern, using Regex maybe?)
        if re.is_match(&num.to_string()).unwrap() {
            matches.push(num);
        }
    }
    return matches;
}
