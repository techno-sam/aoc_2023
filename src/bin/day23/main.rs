use std::{fs, collections::{HashMap, VecDeque, HashSet}};

use char_enum_impl::{char_enum, data_enum};
use utils::{colorize, highlight};

fn main() {
    println!("AOC 2023 Day 23");

    let real = true;
    let contents: String;
    if real {
        contents = fs::read_to_string("src/bin/day23/input.txt").expect("Failed to read input");
    } else {
        contents = fs::read_to_string("src/bin/day23/example.txt").expect("Failed to read example");
    }

    let mut field = Field::load(&contents);
    field.find_intersections();
    field.print();

    println!("\n\n\n");
    let graph: Graph = field.make_graph();
    graph.print_summary();
    let max_distance = graph.max_distance();
    println!("Part 1: {}", max_distance);
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
#[data_enum[(i64, i64)]]
enum Direction {
    North = (0, -1),
    East = (1, 0),
    South = (0, 1),
    West = (-1, 0)
}
impl Direction {
    fn offset(&self, coord: &Coord) -> Coord {
        let (o_column, o_row) = self.value();
        coord.offset(o_row, o_column)
    }

    fn values() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::East,
            Direction::South,
            Direction::West
        ]
    }

    fn opposite(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East
        }
    }

    fn right_and_left(&self) -> Vec<Direction> {
        let mut out = vec![];
        for dir in Direction::values() {
            if dir != *self && dir != self.opposite() {
                out.push(dir);
            }
        }
        return out;
    }
}

#[char_enum]
enum Tile {
    Path = '.',
    Forest = '#',
    SlopeN = '^',
    SlopeE = '>',
    SlopeS = 'v',
    SlopeW = '<'
}
impl Tile {
    /// returns whether this tile can be exited in direction [dir]
    fn can_cross(&self, dir: Direction) -> bool {
        match self {
            Tile::Path => true,
            Tile::Forest => false,

            Tile::SlopeN => dir == Direction::North,
            Tile::SlopeE => dir == Direction::East,
            Tile::SlopeS => dir == Direction::South,
            Tile::SlopeW => dir == Direction::West
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    row: usize,
    column: usize
}
impl Coord {
    fn offset(&self, o_row: i64, o_column: i64) -> Coord {
        let row = (self.row as i64 + o_row) as usize;
        let column = (self.column as i64 + o_column) as usize;
        return Coord { row, column };
    }

    fn unpack(self) -> (usize, usize) {
        return (self.row, self.column);
    }
}

struct TileData {
    tile: Tile,
    intersection: bool,
    visited: bool
}
impl TileData {
    fn decode(chr: char) -> TileData {
        return TileData { tile: Tile::decode(chr), intersection: false, visited: false };
    }
}

struct Field {
    /// [row][column] indexed
    tiles: Vec<Vec<TileData>>,
    width: usize,
    height: usize
}
impl Field {
    fn load(data: &str) -> Field {
        let tiles: Vec<Vec<TileData>> = data.trim().split("\n")
            .map(|l| l.chars()
                .map(|c| TileData::decode(c))
                .collect()
            )
            .collect();
        let width = tiles[0].len();
        let height = tiles.len();
        return Field { tiles, width, height };
    }

    fn find_intersections(&mut self) {
        // exclude border
        for row in 1..(self.height-1) {
            for column in 1..(self.width-1) {
                if let Tile::Forest = self.tiles[row][column].tile {
                    continue;
                }

                let mut adjacent: usize = 0;
                for dir in Direction::values() {
                    let (o_row, o_column) = dir.offset(&Coord { row, column }).unpack();
                    if let Tile::Forest = self.tiles[o_row][o_column].tile {
                        continue;
                    }
                    adjacent += 1;
                }

                if adjacent > 2 {
                    self.tiles[row][column].intersection = true;
                }
            }
        }
    }

    fn print(&self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let hl = self.tiles[row][column].intersection;
                let highlighted = highlight(&format!("{}", self.tiles[row][column].tile.encode()), hl, 0, 255, 0);
                let (r, g, b);
                if hl {
                    (r, g, b) = (0, 0, 0);
                } else if let Tile::Forest = self.tiles[row][column].tile {
                    (r, g, b) = (50, 110, 75);
                } else {
                    (r, g, b) = (255, 255, 255);
                }
                print!("{}", colorize(&highlighted, r, g, b));
            }
            println!();
        }
    }

    fn get(&self, coord: &Coord) -> &TileData {
        return &self.tiles[coord.row][coord.column];
    }

    fn get_mut(&mut self, coord: &Coord) -> &mut TileData {
        return &mut self.tiles[coord.row][coord.column];
    }

    /// Note: [Graph::find_intersections] MUST be called first
    fn make_graph(&mut self) -> Graph {
        let mut intersections: Vec<Coord> = vec![];
        for row in 0..self.height {
            for column in 0..self.width {
                self.tiles[row][column].visited = false;
                if self.tiles[row][column].intersection {
                    intersections.push(Coord { row, column });
                }
            }
        }
        let end_coord = Coord { row: self.height - 1, column: self.width - 2 };
        let mut graph = Graph::new(end_coord, intersections);

        /*
         * Since intersections have been identified, the strategy is thus:
         * have a queue of (origin_coord, travel_dir) for building edges
         */
        let mut queue: VecDeque<(Coord, Direction)> = VecDeque::new();
        self.tiles[0][1].visited = true;
        queue.push_back((Coord { row: 0, column: 1}, Direction::South));
        'Outer: while queue.len() > 0 {
            println!("\n---------------");
            let (start, mut dir) = queue.pop_front().unwrap();
            if self.get(&dir.offset(&start)).visited {
                continue;
            }
            let mut current = start;
            let mut length: usize = 0;

            let mut can_go_forward = true;
            let mut can_go_backward = true;

            'Flood: loop {
                if let Tile::Forest = self.get(&dir.offset(&current)).tile {
                    //println!("Hit forest");
                    let orig = dir;
                    for other_dir in dir.right_and_left() {
                        //println!("Trying {:#?}", other_dir);
                        if let Tile::Forest = self.get(&other_dir.offset(&current)).tile {
                            continue;
                        } else {
                            dir = other_dir;
                            break;
                        }
                    }

                    if orig == dir {
                        println!("{}", colorize(&format!("Dead end starting at {:?} going to {:?}", start, current), 255, 0, 0));
                        continue 'Outer;
                    }
                }
                can_go_forward &= self.get(&current).tile.can_cross(dir);
                can_go_backward &= self.get(&current).tile.can_cross(dir.opposite());
                self.get_mut(&current).visited = true;

                current = dir.offset(&current);
                length += 1;

                if end_coord == current {
                    break 'Flood;
                }

                if self.get(&current).intersection {
                    if !self.get(&current).visited {
                        for o_dir in Direction::values() {
                            if o_dir == dir.opposite() { // we already know not to go backwards
                                continue;
                            }
                            let offset_start = o_dir.offset(&current);
                            if self.get(&offset_start).visited {
                                continue;
                            }
                            if let Tile::Forest = self.get(&offset_start).tile {
                                continue;
                            }
                            queue.push_back((current, o_dir));
                        }
                    }
                    // don't add the same intersection multiple times
                    self.get_mut(&current).visited = true;
                    break 'Flood;
                }
            }

            println!();
            if can_go_forward {
                println!("Forward edge({}) from {:#?} to {:#?}", length, start, current);
                graph.connect(&start, &current, length);
                println!("Connected");
            } else {
                println!("{}", colorize("Cannot go forward", 255, 0, 0));
            }
            if can_go_backward {
                println!("Backward edge({}) from {:#?} to {:#?}", length, current, start);
                graph.connect(&current, &start, length);
                println!("Connected");
            } else {
                println!("{}", colorize("Cannot go backward", 255, 0, 0));
            }
        }

        return graph;
    }
}

struct Node {
    outgoing: Vec<(usize, Coord)>,
    incoming: Vec<(usize, Coord)>
}
impl Node {
    fn new() -> Node {
        return Node { outgoing: vec![], incoming: vec![] };
    }

    fn print_summary(&self) {
        for (len, target) in &self.outgoing {
            println!("  -> [{}] (r: {}, c: {})", len, target.row, target.column);
        }
    }
}

struct Graph {
    end_coord: Coord,
    nodes: HashMap<Coord, Node>
}
impl <'a>Graph {
    fn new(end_coord: Coord, intersections: Vec<Coord>) -> Graph {
        let start = Node::new();
        let end = Node::new();
        let mut nodes: HashMap<Coord, Node> = HashMap::new();
        let start_coord = Coord { row: 0, column: 1 };
        nodes.insert(start_coord, start);
        nodes.insert(end_coord, end);
        for intersection in intersections {
            nodes.insert(intersection, Node::new());
        }
        return Graph { end_coord, nodes };
    }

    #[allow(dead_code)]
    fn toy() -> Graph {
        let start_coord = Coord { row: 0, column: 1 };
        let a = Coord { row: 10, column: 0 };
        let b = Coord { row: 10, column: 20};
        let end_coord = Coord { row: 20, column: 20 };

        /*
         *  /A\
         * S   E
         *  \B/
         */

        let mut nodes: HashMap<Coord, Node> = HashMap::new();
        nodes.insert(start_coord, Node::new());
        nodes.insert(a, Node::new());
        nodes.insert(b, Node::new());
        nodes.insert(end_coord, Node::new());

        let mut graph = Graph { end_coord, nodes };

        graph.connect(&start_coord, &a, 2);
        graph.connect(&start_coord, &b, 5);
        graph.connect(&b, &end_coord, 5);
        graph.connect(&a, &end_coord, 3);

        return graph;
    }

    #[allow(dead_code)]
    #[inline]
    fn start(&'a self) -> &'a Node {
        return self.nodes.get(&Coord { row: 0, column: 1 }).unwrap();
    }

    #[allow(dead_code)]
    #[inline]
    fn end(&'a self) -> &'a Node {
        return self.nodes.get(&self.end_coord).unwrap();
    }

    #[inline]
    fn get(&'a self, coord: &Coord) -> &'a Node {
        return self.nodes.get(coord).unwrap();
    }

    #[inline]
    fn get_mut(&'a mut self, coord: &Coord) -> &'a mut Node {
        return self.nodes.get_mut(coord).unwrap();
    }

    fn connect(&'a mut self, from: &Coord, to: &Coord, dist: usize) {
        self.get_mut(from).outgoing.push((dist, *to));
        self.get_mut(to).incoming.push((dist, *from));
    }

    fn print_summary(&self) {
        println!("Graph summary:");
        println!("  End is: (r: {}, c: {})", self.end_coord.row, self.end_coord.column);
        for (coord, node) in self.nodes.iter() {
            println!("\nNode (r: {}, c: {})", coord.row, coord.column);
            node.print_summary();
        }
    }

    fn recurse_max_distance(&self, visited: HashSet<Coord>, current: Coord, total_length: usize) -> usize {
        let outgoing = self.get(&current).outgoing.iter().filter(|(_, target)| !visited.contains(target));

        let mut max = 0;
        for (edge_length, other_coord) in outgoing {
            let mut new_visited = visited.clone();
            new_visited.insert(current);
            max = max.max(self.recurse_max_distance(new_visited, *other_coord, total_length + edge_length));
        }
        // no intermediate edges
        if max == 0 {
            if current == self.end_coord {
                println!("Reached target with {} steps", total_length);
                return total_length;
            } else {
                println!("Reached dead end");
            }
        }
        return max;
    }

    fn max_distance(&self) -> usize {
        return self.recurse_max_distance(HashSet::new(), Coord { row: 0, column: 1 }, 0);
    }
}

#[test]
fn distance_calc() {
    let g = Graph::toy();
    assert_eq!(10, g.max_distance());
}
