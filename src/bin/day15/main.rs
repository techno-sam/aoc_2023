use std::{fs, env};

fn main() {
    println!("AOC 2023 Day 15");

    let contents = fs::read_to_string("src/bin/day15/input.txt").expect("Failed to read input");
    println!("Part 1: {}", hash_sum(&contents));

    // skip the executable
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {
        println!("Hash of '{}' is {}", arg, hash(&arg));
    }
}

fn hash_sum(data: &str) -> usize {
    return data.trim().split(",").map(|s| hash(s)).sum();
}

fn hash(data: &str) -> usize {
    return data.chars().map(|c| c as usize).fold(0, |acc, e| add_to_hash(acc, e));
}

fn add_to_hash(acc: usize, v: usize) -> usize {
    //println!("Adding {} into existing hash of {}", v, acc);
    let mut current = acc;
    current += v;
    //println!("> after adding, {}", current);
    current *= 17;
    //println!("> after multiplying, {}", current);
    current = current % 256;
    //println!("> after modulo, {}", current);
    return current;
}

#[test]
fn hasher() {
    assert_eq!(52, hash("HASH"));
    assert_eq!(231, hash("ot=7"));
}

#[test]
fn hash_summer() {
    assert_eq!(1320, hash_sum("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"));
}
