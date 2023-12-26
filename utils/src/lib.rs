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

pub trait DijkstraNode<T> where Self: PartialEq + Eq + Hash + Copy {
    /// Returns a vector of (node, distance) pairs
    fn get_connected(&self, context: &T) -> Vec<(Self, usize)> where Self: Sized;
}

pub struct DijkstraData<Node, T> where Node: DijkstraNode<T> {
    unvisited: HashSet<Node>,
    visited: HashSet<Node>,
    pub best_distance: HashMap<Node, usize>,
    pub prev_in_chain: HashMap<Node, Node>,
    context: T
}
impl <Node, T>DijkstraData<Node, T> where Node: DijkstraNode<T> {
    /// note: does NOT add initial to frontier (unvisited nodes)
    fn new(initial: Node, context: T) -> DijkstraData<Node, T> {
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
        return DijkstraData { unvisited, visited, best_distance, prev_in_chain: HashMap::new(), context };
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


    pub fn dijkstra(initial: Node, context: T, should_halt: impl Fn(&Node) -> bool) -> DijkstraData<Node, T> {
        let mut data = DijkstraData::new(initial, context);
        let context = &data.context;
        for (other, distance) in initial.get_connected(context) {
            data.best_distance.insert(other, distance);
            data.prev_in_chain.insert(other, initial);
            data.unvisited.insert(other);
        }

        while let Some(cur) = data.get_best_unvisited() {
            let dist_so_far = *data.best_distance.get(&cur).unwrap();
            for (other, dist) in cur.get_connected(context) {
                if data.visited.contains(&other) {
                    continue;
                }

                data.unvisited.insert(other);
                let new_dist = dist_so_far + dist;
                let (better, best_dist) = match data.best_distance.get(&other) {
                    None => (true, new_dist),
                    Some(&existing) => {
                        if new_dist < existing {
                            (true, new_dist)
                        } else {
                            (false, existing)
                        }
                    }
                };
                if better {
                    data.prev_in_chain.insert(other, cur);
                }
                data.best_distance.insert(other, best_dist);
            }
            data.unvisited.remove(&cur);
            data.visited.insert(cur);
            if should_halt(&cur) {
                return data;
            }
        }

        return data;
    }
}

#[cfg(test)]
#[allow(dead_code, unused_imports)]
mod tests {
    use super::*;
    use char_enum_impl::data_enum;

    // graph from https://www.youtube.com/watch?v=bZkzH5x0SKU
    #[derive(PartialEq, Eq, Hash, Clone, Copy)]
    #[data_enum(Vec<(T, usize)>)]
    enum T {
        A = vec![(T::B, 2), (T::D, 8)],
        B = vec![(T::A, 2), (T::D, 5), (T::E, 6)],
        C = vec![(T::E, 9), (T::F, 3)],
        D = vec![(T::A, 8), (T::B, 5), (T::E, 3), (T::F, 2)],
        E = vec![(T::B, 6), (T::C, 9), (T::D, 2), (T::F, 1)],
        F = vec![(T::C, 3), (T::D, 2), (T::E, 1)]
    }
    impl DijkstraNode<()> for T {
        fn get_connected(&self, _: &()) -> Vec<(Self, usize)> where Self: Sized {
            return self.value();
        }
    }

    #[test]
    fn dijkstra_search() {
        fn hlt(node: &T) -> bool { *node == T::E }
        let a = DijkstraData::dijkstra(T::A, (), hlt);
        assert_eq!(Some(&8_usize), a.best_distance.get(&T::E), "Early halt");

        fn never(_: &T) -> bool { false }
        let a = DijkstraData::dijkstra(T::A, (), never);
        assert_eq!(Some(&12_usize), a.best_distance.get(&T::C), "Halt-less");
    }
}
