use std::fs;

fn main() {
    println!("AOC 2023 Day 2");
    let contents = fs::read_to_string("src/bin/day2/input.txt").expect("Failed to read input.txt");
    let mut total: u32 = 0;
    let mut total_power: u32 = 0;
    for line in contents.split("\n") {
        if line.is_empty() {
            continue;
        }
        let record = parse(line);
        if possible_record(&record, 12, 13, 14) {
            total += record.id;
        }
        total_power += record.power();
    }
    println!("Total sum: {}", total);
    println!("Total power: {}", total_power);
}

struct GameRecord {
    id: u32,
    max_red: u32,
    max_green: u32,
    max_blue: u32
}

impl GameRecord {
    fn power(&self) -> u32 {
        return self.max_red * self.max_green * self.max_blue;
    }
}

// `Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green`
fn parse(line: &str) -> GameRecord {
    let id_records = line.split_once(": ").unwrap();
    let game_id = id_records.0.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();

    let groups = id_records.1.split("; ");
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;

    for group in groups {
        let entries = group.split(", ");
        for entry in entries {
            let (count_str, color) = entry.split_once(" ").unwrap();
            let count = count_str.parse::<u32>().unwrap();
            match color {
                "red" => {
                    if count > max_red {
                        max_red = count;
                    }
                },
                "green" => {
                    if count > max_green {
                        max_green = count;
                    }
                },
                "blue" => {
                    if count > max_blue {
                        max_blue = count;
                    }
                },
                c => panic!("Unknown color {}", c)
            }
        }
    }
    return GameRecord { id: game_id, max_red, max_green, max_blue };
}

#[allow(dead_code)]
fn possible(line: &str, avail_red: u32, avail_green: u32, avail_blue: u32) -> bool {
    return possible_record(&parse(line), avail_red, avail_green, avail_blue);
}

fn possible_record(record: &GameRecord, avail_red: u32, avail_green: u32, avail_blue: u32) -> bool {
    return record.max_red <= avail_red && record.max_green <= avail_green && record.max_blue <= avail_blue;
}

#[test]
fn parsing() {
    let g = parse("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");
    assert_eq!(g.id, 1);
    assert_eq!(g.max_red, 4);
    assert_eq!(g.max_green, 2);
    assert_eq!(g.max_blue, 6);
    assert_eq!(g.power(), 48);
}

#[test]
fn possibility() {
    assert_eq!(true, possible("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green", 12, 13, 14));
    assert_eq!(true, possible("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue", 12, 13, 14));
    assert_eq!(false, possible("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red", 12, 13, 14));
    assert_eq!(false, possible("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red", 12, 13, 14));
    assert_eq!(true, possible("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green", 12, 13, 14));
}
