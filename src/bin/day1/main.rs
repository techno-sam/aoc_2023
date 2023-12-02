use std::fs;

fn main() {
    println!("AOC 2023 Day 1");
    let contents = fs::read_to_string("src/bin/day1/input.txt").expect("Failed to read input.txt");
    let mut total1 = 0;
    let mut total2 = 0;
    for line in contents.split("\n") {
        if !line.is_empty() {
            total1 += extract1(line);
            total2 += extract2(line);
        }
    }
    println!("Sum of calibration values (part 1): {}", total1);
    println!("Sum of calibration values (part 2): {}", total2);
}

fn extract1(line: &str) -> u32 {
    return extract(line, false);
}

fn extract2(line: &str) -> u32 {
    return extract(line, true);
}

fn extract(line: &str, include_words: bool) -> u32 {
    let mut first: i32 = -1;
    let mut last: i32 = -1;
    let mut chars = line.chars();
    let mut idx: usize = 0;
    loop {
        let char = match chars.next() {
            Some(c) => c,
            None => break
        };
        idx += 1;
        let digit = char.to_digit(10);
        match digit {
            Some(d) => {
                if first == -1 {
                    first = d as i32;
                } else {
                    last = d as i32;
                }
                continue;
            },
            None => {
                if include_words {
                    match extract_word_num(&line[idx-1..]) {
                        Some(d) => {
                            if first == -1 {
                                first = d as i32;
                            } else {
                                last = d as i32;
                            }
                            continue;
                        },
                        None => {},
                    }
                }
            },
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

fn extract_word_num(txt: &str) -> Option<u32> {
    // println!("Trying to decode \"{}\"", txt);
    let digits = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let count = txt.chars().count();
    let mut digit: u32 = 0;
    'Inner: for digit_word in digits {
        digit += 1;
        let dcount = digit_word.chars().count();
        if dcount > count {
            continue 'Inner;
        }
        for offset in 0..dcount {
            if txt.chars().nth(offset).unwrap() != digit_word.chars().nth(offset).unwrap() {
                continue 'Inner;
            }
        }
        return Some(digit);
    }
    return None;
}


#[test]
fn extraction_part1() {
    assert_eq!(12, extract1("1abc2"));
    assert_eq!(38, extract1("pqr3stu8vwx"));
    assert_eq!(15, extract1("a1b2c3d4e5f"));
    assert_eq!(77, extract1("treb7uchet"));
}

#[test]
fn extracts_words() {
    assert_eq!(None, extract_word_num("aone"));
    assert_eq!(Some(2), extract_word_num("twosomethingelsethree"));
}

#[test]
fn extraction_part2() {
    assert_eq!(12, extract2("1abc2"));
    assert_eq!(38, extract2("pqr3stu8vwx"));
    assert_eq!(15, extract2("a1b2c3d4e5f"));
    assert_eq!(77, extract2("treb7uchet"));

    assert_eq!(29, extract2("two1nine"));
    assert_eq!(83, extract2("eighttwothree"));
    assert_eq!(14, extract2("zoneight234"));
}
