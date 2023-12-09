use std::{collections::HashMap, fs};

fn main() {
    println!("AOC 2023 Day 8");

    let contents = fs::read_to_string("src/bin/day8/input.txt").expect("Failed to load input");
    let (pattern, network_data) = contents.split_once("\n\n").unwrap();
    let network = Network::load(network_data);
    let steps = network.traverse(pattern);
    println!("Part 1 steps: {}", steps);
    let steps2 = network.lcm_all(pattern);
    println!("Part 2 steps: {}", steps2);
}

type NodeID = String;

#[derive(Debug)]
struct Network {
    connections: HashMap<NodeID, (NodeID, NodeID)>,
    starting_nodes: Vec<NodeID>
}
#[allow(dead_code)]
impl Network {
    fn load(data: &str) -> Network {
        let mut connections: HashMap<NodeID, (NodeID, NodeID)> = HashMap::new();
        let mut starting_nodes: Vec<NodeID> = vec![];
        let lines = data.trim().split("\n")
            .map(|l| l.split_once(" = ").unwrap());
        let split_more = lines
            .map(|(id, ids)| {
                let cleaned = ids.replace("(", "").replace(")", "");
                let split_ids: (&str, &str) = cleaned.split_once(", ").unwrap();
                return (id, (split_ids.0.to_owned(), split_ids.1.to_owned()));
            });
        for (id, (left, right)) in split_more {
            connections.insert(id.to_owned(), (left.to_owned(), right.to_owned()));
            if id.ends_with("A") {
                starting_nodes.push(id.to_owned());
            }
        }
        return Network { connections, starting_nodes };
    }

    fn traverse(&self, pattern: &str) -> u64 {
        let mut steps: u64 = 0;
        let mut id: NodeID = "AAA".to_string();
        let pattern_length = pattern.chars().count();
        while id != "ZZZ" {
            let direction = pattern.chars().nth((steps as usize) % pattern_length).unwrap();
            // println!("Beginning of loop, steps={}, direction={}", steps, direction);
            steps += 1;
            id = match direction {
                'L' => self.connections.get(&id).unwrap().0.to_owned(),
                'R' => self.connections.get(&id).unwrap().1.to_owned(),
                _ => panic!("Invalid direction {}", direction)
            };
        }
        return steps;
    }

    /// @return: cycle length
    fn traverse_one_endings(&self, pattern: &str, starting_id: NodeID) -> u64 {
        let mut steps: u64 = 0;
        let mut id: NodeID = starting_id;
        let pattern_length = pattern.chars().count();
        while !id.ends_with("Z") || (steps as usize - 1) % pattern_length == 0 {
            let direction = pattern.chars().nth((steps as usize) % pattern_length).unwrap();
            // println!("Beginning of loop, steps={}, direction={}", steps, direction);
            steps += 1;
            id = match direction {
                'L' => self.connections.get(&id).unwrap().0.to_owned(),
                'R' => self.connections.get(&id).unwrap().1.to_owned(),
                _ => panic!("Invalid direction {}", direction)
            };
        }
        return steps;
    }

    fn lcm_all(&self, pattern: &str) -> u64 {
        println!("Starting to traverse {} starting nodes", self.starting_nodes.len());
        let cycles: Vec<u64> = self.starting_nodes.iter().map(|start| self.traverse_one_endings(pattern, start.to_owned())).collect();
        println!("Mapped cycles {:?}", cycles);
        let reduced = cycles.iter().map(|v| *v).reduce(|acc, e| {
            let least = lcm(acc, e);
            println!("Reducing ({}, {}) -> {}", acc, e, least);
            return least;
        });
        println!("Reduced");
        return reduced.unwrap();
    }

    fn traverse_all(&self, pattern: &str) -> u64 {
        let mut steps: u64 = 0;
        let mut ids: Vec<NodeID> = self.starting_nodes.clone();
        let pattern_length = pattern.chars().count();
        // idea for faster implementation: when
        // (steps % pattern_length == 0) && ids[i] == starting[i]
        // then add that to a list & find LCM
        while !ids.iter().all(|id| id.ends_with("Z")) {
            let direction = pattern.chars().nth((steps as usize) % pattern_length).unwrap();
            // println!("Beginning of loop, steps={}, direction={}", steps, direction);
            steps += 1;
            for (i, id) in ids.clone().iter().enumerate() {
                ids[i] = match direction {
                    'L' => self.connections.get(id).unwrap().0.to_owned(),
                    'R' => self.connections.get(id).unwrap().1.to_owned(),
                    _ => panic!("Invalid direction {}", direction)
                };
            }
        }
        return steps;
    }
}

fn prime_factorize(num: u64) -> Vec<u64> {
    let mut v = num;
    let mut factors: Vec<u64> = vec![];
    let max = (v as f64).sqrt().round() as u64;
    for x in 2..=max+2 {
        while v%x == 0 {
            factors.push(x);
            v /= x;
        }
    }
    if v > 1 {
        factors.push(v);
    }
    return factors;
}

fn lcm(a: u64, b: u64) -> u64 {
    let mut counts_a: HashMap<u64, u64> = HashMap::new();
    let mut counts_b: HashMap<u64, u64> = HashMap::new();
    for factor_a in prime_factorize(a) {
        counts_a.insert(factor_a, counts_a.get(&factor_a).unwrap_or(&0) + 1);
    }
    for factor_b in prime_factorize(b) {
        counts_b.insert(factor_b, counts_b.get(&factor_b).unwrap_or(&0) + 1);
    }
    println!("lcm({}, {}):", a, b);
    println!("\tcounts_a: {:?}", counts_a);
    println!("\tcounts_b: {:?}", counts_b);
    for (factor, count) in counts_b.iter() {
        counts_a.insert(*factor, *counts_a.get(factor).unwrap_or(count).max(count));
    }
    return counts_a.iter().map(|(factor, count)| factor.pow(*count as u32)).product();
}

#[allow(dead_code)]
fn get_test_network() -> (String, Network) {
    let contents = fs::read_to_string("src/bin/day8/test.txt").expect("Failed to load test.txt");
    let (pattern, network_data) = contents.split_once("\n\n").unwrap();
    return (pattern.to_owned(), Network::load(network_data));
}

#[allow(dead_code)]
fn get_test_network_2() -> (String, Network) {
    let contents = fs::read_to_string("src/bin/day8/test2.txt").expect("Failed to load test.txt");
    let (pattern, network_data) = contents.split_once("\n\n").unwrap();
    return (pattern.to_owned(), Network::load(network_data));
}

#[test]
fn network_traversal() {
    let (pattern, network) = get_test_network();
    assert_eq!(6, network.traverse(&pattern));
}

#[test]
fn prime_factorize_works() {
    assert_eq!(vec![2, 3, 7], prime_factorize(42));
    assert_eq!(vec![2, 2, 3, 5], prime_factorize(60));
    assert_eq!(vec![59, 307], prime_factorize(18113));
    assert_eq!(420, lcm(42, 60));
}

#[test]
fn network_traversal_2() {
    let (pattern, network) = get_test_network_2();
    assert_eq!(6, network.lcm_all(&pattern));
}
