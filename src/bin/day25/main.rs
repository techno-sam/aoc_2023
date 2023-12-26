use std::{fs, collections::{HashMap, HashSet}};

fn main() {
    println!("AOC 2023 Day 25");

    let real = false;
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

    let a: usize;
    let b: usize;
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
    }

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
