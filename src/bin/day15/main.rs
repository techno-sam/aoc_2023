use std::{fs, env};
use arr_macro::arr;

fn main() {
    println!("AOC 2023 Day 15");

    let contents = fs::read_to_string("src/bin/day15/input.txt").expect("Failed to read input");
    println!("Part 1: {}", hash_sum(&contents));

    // skip the executable
    let args: Vec<String> = env::args().skip(1).collect();
    for arg in args {
        println!("Hash of '{}' is {}", arg, hash(&arg));
    }

    let hm: HashMap = HashMap::parse(&contents);
    println!("Part 2: {}", hm.focusing_power());
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

enum Instruction {
    Remove(String),
    Add(String, u8)
}
impl Instruction {
    fn decode(data: &str) -> Instruction {
        match data.split_once("=") {
            Some((label, focal_length_str)) => {
                return Instruction::Add(label.to_owned(), focal_length_str.parse::<u8>().unwrap())
            },
            None => {
                assert!(data.ends_with("-"));
                return Instruction::Remove(data.strip_suffix("-").unwrap().to_owned());
            }
        }
    }
}

struct LensPair {
    label: String,
    focal_length: u8
}
impl LensPair {
    fn from(label: String, focal_length: u8) -> LensPair {
        return LensPair { label, focal_length };
    }
}

struct HashMap {
    backing: [Vec<LensPair>; 256]
}
impl HashMap {
    fn parse(instructions: &str) -> HashMap {
        let mut hm = HashMap { backing: arr![Vec::new(); 256] };
        let parsed = instructions.trim().split(",").map(|s| Instruction::decode(s));
        parsed.for_each(|i| hm.handle(i));
        return hm;
    }

    fn handle(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Remove(label) => self.remove(&label),
            Instruction::Add(label, focal_length) => self.add(&label, focal_length)
        };
    }

    fn remove(&mut self, label: &str) {
        let bucket_id = hash(label);
        let bucket = &mut self.backing[bucket_id];

        let idx = bucket.iter().enumerate().filter(|(_, pair)| pair.label == label).map(|(i, _)| i).last();
        if let Some(i) = idx {
            bucket.remove(i);
        }
    }

    fn add(&mut self, label: &str, focal_length: u8) {
        let bucket_id = hash(label);
        let bucket = &mut self.backing[bucket_id];

        let idx = bucket.iter().enumerate().filter(|(_, pair)| pair.label == label).map(|(i, _)| i).last();
        if let Some(i) = idx { // replace existing
            bucket[i] = LensPair::from(label.to_owned(), focal_length);
        } else { // add to end
            bucket.push(LensPair::from(label.to_owned(), focal_length));
        }
    }

    fn focusing_power(&self) -> usize {
        let mut sum = 0;
        for (box_id, bucket) in self.backing.iter().enumerate() {
            for (lens_id, pair) in bucket.iter().enumerate() {
                sum += (box_id+1) * (lens_id + 1) * (pair.focal_length as usize);
            }
        }
        return sum;
    }
}

#[test]
fn instruction_parsing() {
    assert_eq!(145, HashMap::parse("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7").focusing_power());
}
