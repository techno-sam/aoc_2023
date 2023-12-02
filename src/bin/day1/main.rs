use std::fs;

fn main() {
    println!("AOC 2023 Day 1");
    let contents = fs::read_to_string("src/bin/day1/input.txt").expect("Failed to read input.txt");
    let mut total = 0;
    for line in contents.split("\n") {
        if !line.is_empty() {
            total += extract(line);
        }
    }
    println!("Sum of calibration values: {}", total);
}

fn extract(line: &str) -> u32 {
    let mut first: i32 = -1;
    let mut last: i32 = -1;
    let mut chars = line.chars();
    loop {
        let char = match chars.next() {
            Some(c) => c,
            None => break
        };
        let digit = char.to_digit(10);
        match digit {
            Some(d) => {
                if first == -1 {
                    first = d as i32;
                } else {
                    last = d as i32;
                }
            },
            None => {},
        }
    }
    if first == -1 {
        panic!("No digits found on line \"{}\"", line);
    }
    if last == -1 {
        last = first;
    };
    return ((first * 10) + last) as u32;
}


#[test]
fn extraction() {
    assert_eq!(12, extract("1abc2"));
    assert_eq!(38, extract("pqr3stu8vwx"));
    assert_eq!(15, extract("a1b2c3d4e5f"));
    assert_eq!(77, extract("treb7uchet"));
}
