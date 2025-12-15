use std::env;
use std::fs;

fn main() {

    // Processing file: at the end have a Vec of &str which we can individually traverse.
    let args: Vec<String> = env::args().collect();
    let file_path = &args[2];
    let contents: String =
        fs::read_to_string(file_path)
        .expect("Should have been able to read the file.")
        .trim().to_string();
    // Note: Because to_string is binding. If we do .split().collect() after, there is nothing that can own the original long String so we can't reference to it.
    // Hence why we need contents (owner of original value) and content_lines (reference to contents) as SEPARATE values.
    let content_lines: Vec<&str> = contents.split("\n").collect();
 
    // for line in content_lines {
        
    // }

}

// find_max_two_digit_num(bank: &str) -> u32 {

// }