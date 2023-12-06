use std::fs;

use aho_corasick::AhoCorasick;

fn main() {
    let content: Vec<String> = fs::read_to_string("./src/input2.txt")
        .expect("not able to open file")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    let dictionary = &[
        "one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7",
        "eight", "8", "nine", "9",
    ];

    let ac = AhoCorasick::new(dictionary).unwrap();
    let mut total = 0;
    for line in content.iter() {
        let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();
        let first = matches.first().unwrap().pattern().as_i32() / 2 + 1;
        let last = matches.last().unwrap().pattern().as_i32() / 2 + 1;

        total += (first * 10) + last;
    }
    println!("{}",total);
}
