use std::{fs, collections::{HashMap, HashSet}, hash::{Hash, Hasher}, fmt::{Debug, Display}};

use rand::seq::SliceRandom;
use utils::{DijkstraNode, DijkstraData};

fn main() {
    println!("AOC 2023 Day 25");

    let real = true;
    let contents: String;
    if real {
        contents = fs::read_to_string("src/bin/day25/input.txt").expect("Failed to read input");
    } else {
        contents = fs::read_to_string("src/bin/day25/example.txt").expect("Failed to read example");
    }

    let mut graph = Graph::new();
    for line in contents.trim().split("\n") {
        let (from, to) = line.split_once(": ").unwrap();
        let to = to.split(" ");
        for t in to {
            graph.join(to_label(from), to_label(t));
        }
    }

    let labels: Vec<Label> = graph.nodes.keys().map(|l| *l).collect();
    let rng = &mut rand::thread_rng();
    // Find cut-nodes
    let mut edge_count: HashMap<(Label, Label), usize> = HashMap::new();
    for _ in 0..1000 {
        let a = labels.choose(rng).unwrap();
        let b = labels.choose(rng).unwrap();
        if a == b {
            continue;
        }

        if !real {
            println!("a: {}, b: {}", to_string(a), to_string(b));
        }

        let a = DijkstraCompatNode { graph: &graph, lbl: *a };
        let b = DijkstraCompatNode { graph: &graph, lbl: *b };

        let hlt = |node: &DijkstraCompatNode<'_>| -> bool {
            return b.lbl == node.lbl;
        };

        let d = DijkstraData::dijkstra(a, (), hlt);
        //println!("{}", d.best_distance.get(&b).unwrap());
        /*for (k, v) in &d.prev_in_chain {
            println!("{} <- {}", v, k);
        }*/
        let mut curr = &b;
        loop {
            let prev = match d.prev_in_chain.get(curr) {
                None => break,
                Some(v) => v
            };
            let first = curr.lbl.min(prev.lbl);
            let second = curr.lbl.max(prev.lbl);

            edge_count.insert((first, second), 1 + match edge_count.get(&(first, second)) {
                None => 0,
                Some(v) => *v
            });

            curr = prev;
        }
        //println!("{}", to_string(&d.prev_in_chain.get(&b).unwrap().lbl));
        //println!("\n\n\n\n");
    }

    let mut to_cut: Vec<(&(Label, Label), &usize)> = edge_count.iter().collect();
    to_cut.sort_unstable_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());

    println!("\n\n\n");

    let a: usize;
    let b: usize;
    /*
    // just use GraphViz in 'neato' mode to determine the cuts
    if real {
        graph.cut(to_label("klk"), to_label("xgz"));
        graph.cut(to_label("vmq"), to_label("cbl"));
        graph.cut(to_label("bvz"), to_label("nvf"));

        a = graph.count_from(to_label("klk"));
        b = graph.count_from(to_label("xgz"));
    } else {
        graph.cut(to_label("hfx"), to_label("pzl"));
        graph.cut(to_label("bvb"), to_label("cmg"));
        graph.cut(to_label("nvd"), to_label("jqt"));

        a = graph.count_from(to_label("hfx"));
        b = graph.count_from(to_label("pzl"));
    }*/
    for i in 0..3 {
        let ((from, to), _) = to_cut[i];
        println!("Should cut: {}, {}", to_string(from), to_string(to));
        graph.cut(*from, *to);
    }
    a = graph.count_from(to_cut[0].0.0);
    b = graph.count_from(to_cut[0].0.1);

    println!("A: {}", a);
    println!("B: {}", b);
    println!("Part 1: {}", a*b);

    if !real {
        for (lbl, node) in graph.nodes.iter() {
            println!("Node {:?} [{}] -> {:?}", to_string(lbl), node.visited, node.connections.iter()
                .map(|l| to_string(l)).collect::<Vec<_>>().join(", "));
        }

        println!("debug graph:\n\n");
        for (_, node) in graph.nodes.iter() {
            node.print_debug();
        }
    }
}

type Label = [char; 3];

fn to_label(data: &str) -> Label {
    let mut chars = ['.'; 3];
    let mut chr_iter = data.chars();
    for i in 0..3 {
        chars[i] = chr_iter.next().unwrap();
    }
    return chars;
}

fn to_string(lbl: &Label) -> String {
    return String::from_iter(lbl.iter());
}

#[derive(Clone, Copy)]
struct DijkstraCompatNode<'a> {
    graph: &'a Graph,
    lbl: Label
}
impl <'a>DijkstraCompatNode<'a> {
    fn with_label(&self, lbl: Label) -> DijkstraCompatNode<'a> {
        return DijkstraCompatNode { graph: self.graph, lbl };
    }
}
impl PartialEq for DijkstraCompatNode<'_> {
    fn eq(&self, other: &Self) -> bool {
        return self.lbl == other.lbl;
    }
}
impl Eq for DijkstraCompatNode<'_> {}
impl Hash for DijkstraCompatNode<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.lbl.hash(state);
    }
}
impl Display for DijkstraCompatNode<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&to_string(&self.lbl))
    }
}
impl <'a>DijkstraNode<()> for DijkstraCompatNode<'a> {
    fn get_connected(&self, _: &()) -> Vec<(Self, usize)> where Self: Sized {
        return self.graph.get(self.lbl).connections.iter().map(|c| (self.with_label(*c), 1_usize)).collect();
    }
}

struct Node {
    #[allow(dead_code)]
    lbl: Label,
    connections: HashSet<Label>,
    visited: bool
}
impl Node {
    fn new(lbl: Label) -> Node {
        return Node { lbl, connections: HashSet::new(), visited: false };
    }

    fn print_debug(&self) {
        print!("{}:", to_string(&self.lbl));
        for con in &self.connections {
            print!(" {}", to_string(con));
        }
        println!();
    }
}

struct Graph {
    nodes: HashMap<Label, Node>
}
impl Graph {
    fn new() -> Graph {
        return Graph { nodes: HashMap::new() };
    }

    fn join(&mut self, a: Label, b: Label) {
        if !self.nodes.contains_key(&a) {
            self.nodes.insert(a, Node::new(a));
        }
        if !self.nodes.contains_key(&b) {
            self.nodes.insert(b, Node::new(b));
        }
        self.nodes.get_mut(&a).unwrap().connections.insert(b);
        self.nodes.get_mut(&b).unwrap().connections.insert(a);
    }

    fn cut(&mut self, a: Label, b: Label) {
        self.nodes.get_mut(&a).unwrap().connections.remove(&b);
        self.nodes.get_mut(&b).unwrap().connections.remove(&a);
    }

    fn get(&self, lbl: Label) -> &Node {
        return self.nodes.get(&lbl).unwrap();
    }

    fn get_mut(&mut self, lbl: Label) -> &mut Node {
        return self.nodes.get_mut(&lbl).unwrap();
    }

    fn count_from(&mut self, lbl: Label) -> usize {
        let mut sum = 1;
        let mut frontier: Vec<Label> = vec![];

        self.get_mut(lbl).visited = true;
        for connection in &self.get(lbl).connections {
            frontier.push(*connection);
        }

        while frontier.len() > 0 {
            let label = frontier.pop().unwrap();
            if self.get(label).visited {
                continue;
            }
            println!("Handling: {}", to_string(&label));
            sum += 1;

            self.get_mut(label).visited = true;
            for connection in &self.get(label).connections {
                if !self.get(*connection).visited {
                    frontier.push(*connection);
                }
            }
        }
        return sum;
    }
}
