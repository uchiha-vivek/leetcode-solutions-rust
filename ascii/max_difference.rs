// maximum difference in a string
use std::io::{self, Read};

fn main() {
    // Read input string
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.trim();

    // Edge case: if string has < 2 chars, max difference = 0
    if s.len() < 2 {
        println!("0");
        return;
    }

    let bytes = s.as_bytes();
    let mut max_diff: i32 = 0;

    for i in 0..bytes.len() - 1 {
        let diff = (bytes[i] as i32 - bytes[i + 1] as i32).abs();
        if diff > max_diff {
            max_diff = diff;
        }
    }

    println!("{}", max_diff);
}
