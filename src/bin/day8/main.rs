use std::{collections::HashMap, fs};

fn main() {
    println!("AOC 2023 Day 8");

    let contents = fs::read_to_string("src/bin/day8/input.txt").expect("Failed to load input");
    let (pattern, network_data) = contents.split_once("\n\n").unwrap();
    let network = Network::load(network_data);
    let steps = network.traverse(pattern);
    println!("Part 1 steps: {}", steps);
}

type NodeID = String;

#[derive(Debug)]
struct Network {
    connections: HashMap<NodeID, (NodeID, NodeID)>
}
impl Network {
    fn load(data: &str) -> Network {
        let mut connections: HashMap<NodeID, (NodeID, NodeID)> = HashMap::new();
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
        }
        return Network { connections };
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
}

#[allow(dead_code)]
fn get_test_network() -> (String, Network) {
    let contents = fs::read_to_string("src/bin/day8/test.txt").expect("Failed to load test.txt");
    let (pattern, network_data) = contents.split_once("\n\n").unwrap();
    return (pattern.to_owned(), Network::load(network_data));
}

#[test]
fn network_traversal() {
    let (pattern, network) = get_test_network();
    assert_eq!(6, network.traverse(&pattern));
}
