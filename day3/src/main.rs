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
    let mut sum = 0;

    for line in content_lines {
        let max_digit = find_max_two_digit_num(line);
        sum += max_digit;
    }
    println!("{sum}");
}

fn find_max_two_digit_num(bank: &str) -> u32 {
    let integers: Vec<u32> = bank
    .chars()
    .filter_map(|c| c.to_digit(10))
    .collect();

    // Getting first digit of max digit (which will be maximum digit from first element to second to last)
    let mut max_index = 0;
    for (idx, &value) in integers[0..=integers.len()-2].iter().enumerate() {
        if value > integers[max_index] {
            max_index = idx;
        }
    }
    let first_digit = integers[max_index];

    // Getting second digit of max digit.
    let second_digit_slice: &[u32] = &integers[max_index+1..];
    let second_digit: &u32 = second_digit_slice.iter().max().unwrap();

    return (first_digit*10) + second_digit;
}