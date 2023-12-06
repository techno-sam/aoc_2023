use std::fs;

fn main() {
    println!("AOC 2023 Day 6");
    test_main();

    let contents = fs::read_to_string("src/bin/day6/input.txt").expect("Failed to read input");
    let (times, distances) = contents.trim().split_once("\n").unwrap();

    let times_iter = times.split(" ")
        .filter(|s| !s.is_empty())
        .filter(|s| &"Time:" != s)
        .map(|s| s.parse::<u64>().unwrap());

    let distances_iter = distances.split(" ")
        .filter(|s| !s.is_empty())
        .filter(|s| &"Distance:" != s)
        .map(|s| s.parse::<u64>().unwrap());

    let races: Vec<Race> = times_iter.zip(distances_iter).map(|(time, distance)| Race { time, distance }).collect();
    let variance_product: u64 = races.iter()
        .map(|race| possibilities(race.time, race.distance))
        .product();
    println!("Part 1 variance product: {}", variance_product);

    let kerned_time: u64 = times.replace("Time:", "").replace(" ", "").parse::<u64>().unwrap();
    let kerned_distance: u64 = distances.replace("Distance:", "").replace(" ", "").parse::<u64>().unwrap();
    let variance_2: u64 = possibilities(kerned_time, kerned_distance);
    println!("Part 2 kerned variance: {}", variance_2);
}

fn test_main() {
    let time = 30;
    let record = 200;
    let lower = bound(time, record, true);
    let upper = bound(time, record, false);
    println!("\nFor {} ms race with record {} mm:", time, record);
    println!("\tLower bound {} ms ({} mm)", lower, distance_traveled(lower, time));
    println!("\tUpper bound {} ms ({} mm)\n\n", upper, distance_traveled(upper, time));
}

struct Race {
    time: u64,
    distance: u64
}

fn distance_traveled(button_time: u64, race_time: u64) -> u64 {
    // movement speed = button time
    let movement_time: u64 = race_time - button_time;
    return movement_time * button_time;
}

fn bound(race_time: u64, record_distance: u64, lower: bool) -> u64 {
    let y: f64 = race_time as f64;
    let d: f64 = record_distance as f64;
    let y_over_2: f64 = y / 2f64;
    let y_over_2_sqr: f64 = y_over_2 * y_over_2;
    let sqrt_y_d: f64 = (y_over_2_sqr - d).sqrt();
    const EPSILON: f64 = 0.0001;
    if lower {
        return (-sqrt_y_d + y_over_2 + EPSILON).ceil() as u64;
    } else {
        return (sqrt_y_d + y_over_2 - EPSILON).floor() as u64;
    }
}

fn possibilities(race_time: u64, record_distance: u64) -> u64 {
    let lower = bound(race_time, record_distance, true);
    let upper = bound(race_time, record_distance, false);
    assert!(distance_traveled(lower, race_time) > record_distance, "sanity check");
    assert!(distance_traveled(upper, race_time) > record_distance, "sanity check");
    return (upper - lower) + 1;
}

#[test]
fn possibility_count() {
    assert_eq!(4, possibilities(7, 9));
    assert_eq!(8, possibilities(15, 40));
    assert_eq!(9, possibilities(30, 200));
}
