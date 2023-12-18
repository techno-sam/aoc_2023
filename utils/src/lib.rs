use std::{collections::{HashSet, HashMap}, hash::Hash};

pub fn colorize(input: &str, r: u8, g: u8, b: u8) -> String {
    return "\x1b[38;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

pub fn highlight(input: &str, actually: bool, r: u8, g: u8, b: u8) -> String {
    if !actually {
        return input.to_owned();
    }
    return "\x1b[48;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

pub trait DijkstraNode {
    /// Returns a vector of (node, distance) pairs
    fn get_connected(&self) -> Vec<(Self, usize)> where Self: Sized;
}

pub struct DijkstraData<Node> where Node: PartialEq + Eq + Hash + Copy + DijkstraNode {
    unvisited: HashSet<Node>,
    visited: HashSet<Node>,
    best_distance: HashMap<Node, usize>
}
impl <Node>DijkstraData<Node> where Node: PartialEq + Eq + Hash + Copy + DijkstraNode {
    /// note: does NOT add initial to frontier (unvisited nodes)
    fn new(initial: Node) -> DijkstraData<Node> {
        let unvisited = HashSet::new();
        let visited = {
            let mut visited = HashSet::new();
            visited.insert(initial);
            visited
        };
        let best_distance = {
            let mut best_distance = HashMap::new();
            best_distance.insert(initial, 0);
            best_distance
        };
        return DijkstraData { unvisited, visited, best_distance };
    }

    fn get_best_unvisited(&self) -> Option<Node> {
        if self.unvisited.len() == 0 {
            return None;
        }
        let mut best: Option<Node> = None;
        let mut best_distance = usize::max_value();
        for node in &self.unvisited {
            let dist = *self.best_distance.get(node).expect("Missing best distance for unvisited point");
            //.unwrap_or(&usize::max_value());
            if dist <= best_distance {
                best_distance = dist;
                best = Some(*node);
            }
        }

        return best;
    }


    pub fn dijkstra(initial: Node) -> DijkstraData<Node> {
        let mut data = DijkstraData::new(initial);
        for (other, distance) in initial.get_connected() {
            data.best_distance.insert(other, distance);
            data.unvisited.insert(other);
        }

        while let Some(cur) = data.get_best_unvisited() {
            let dist_so_far = *data.best_distance.get(&cur).unwrap();
            for (other, dist) in cur.get_connected() {
                if data.visited.contains(&other) {
                    continue;
                }

                data.unvisited.insert(other);
                let best_dist = match data.best_distance.get(&other) {
                    None => dist_so_far + dist,
                    Some(&existing) => existing.min(dist_so_far + dist)
                };
                data.best_distance.insert(other, best_dist);
            }
            data.unvisited.remove(&cur);
            data.visited.insert(cur);
        }

        return data;
    }
}
