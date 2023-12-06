use std::fs;

fn main() {
    let content: Vec<String> = fs::read_to_string("./src/input1.txt")
        .expect("not able to open file")
        .split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();
    let mut total = 0;

    for line in content {
        let numbers = line
            .chars()
            .filter(|ch| ch.is_ascii_digit())
            .map(|ch| ch as u8 - b'0')
            .collect::<Vec<_>>();
        let first = numbers.first().unwrap();
        let last = numbers.last().unwrap();
        total += (*first as i32) * 10 + (*last as i32);
    }
    println!("{total}");
}
