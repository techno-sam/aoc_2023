use std::{fs, collections::HashSet};

use char_enum_impl::char_enum;

fn main() {
    println!("AOC 2023 Day 21");

    let contents = fs::read_to_string("src/bin/day21/input.txt").expect("Failed to read input");

    let mut field = Field::load(&contents);

    let total = field.flood(64);
    println!("Part 1: {}", total);
}

#[allow(dead_code)]
fn get_test_input() -> Field {
    return Field::load("
...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
");
}

#[derive(Clone, Copy)]
#[char_enum]
enum Tile {
    Garden = '.',
    Rocks = '#',
    Start = 'S',
    Explored = 'O' // doesn't actually show up in input, but useful for visualization
}
impl Tile {
    fn can_enter(&self) -> bool {
        match self {
            Tile::Garden => true,
            Tile::Rocks => false,
            Tile::Start => true,
            Tile::Explored => false // this could probably be true, but whatever
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
}

struct Field {
    /// [row][column] indexed
    tiles: Vec<Vec<Tile>>,
    width: usize,
    height: usize,
    start_coord: Coord
}
impl Field {
    fn load(data: &str) -> Field {
        let lines: Vec<&str> = data.trim().split("\n").collect();
        let width = lines[0].len();
        let height = lines.len();

        let tiles: Vec<Vec<Tile>> = lines.iter()
            .map(|l| l.chars()
                .map(|c| Tile::decode(c))
                .collect()
            )
            .collect();

        let mut start_coord: Option<Coord> = None;

        'Outer: for row in 0..height {
            for column in 0..width {
                if let Tile::Start = tiles[row][column] {
                    start_coord = Some(Coord { row, column });
                    break 'Outer;
                }
            }
        }

        let start_coord: Coord = start_coord.expect("No starting coordinate found");

        return Field { tiles, width, height, start_coord };
    }

    fn valid_offsets(&self, coord: Coord) -> Vec<Coord> {
        let mut out: Vec<Coord> = vec![];

        for o_row in -1..=1_i64 {
            for o_column in -1..=1_i64 {
                if o_row == 0 && o_column == 0 { // skip 'identity'
                    continue;
                }
                if o_row.abs() + o_column.abs() >= 2 { // don't go diagonally
                    continue;
                }

                if coord.row == 0 && o_row == -1 {
                    continue;
                }

                if coord.column == 0 && o_column == -1 {
                    continue;
                }

                let other = coord.offset(o_row, o_column);
                if other.row >= self.height || other.column >= self.width {
                    continue;
                }
                out.push(other);
            }
        }

        return out;
    }

    fn print(&self) {
        for row in 0..self.height {
            for column in 0..self.width {
                print!("{}", self.tiles[row][column].encode());
            }
            println!("");
        }
    }

    fn flood(&mut self, steps: usize) -> usize {
        println!("Initial state:");
        self.print();

        let mut total_others: HashSet<Coord> = HashSet::new();

        let mut current_frontier: HashSet<Coord> = HashSet::new();//vec![self.start_coord];
        current_frontier.insert(self.start_coord);
        let mut next_frontier: HashSet<Coord> = HashSet::new();

        for step in 1..=steps {
            let mut restore: Vec<(Coord, Tile)> = vec![];
            for coord in current_frontier {
                let other_coords = self.valid_offsets(coord);

                for other in other_coords {
                    let other_tile = &self.tiles[other.row][other.column];
                    if other_tile.can_enter() {
                        if step == steps { // only count last step
                            total_others.insert(other);
                        }
                        next_frontier.insert(other);
                        restore.push((other, self.tiles[other.row][other.column]));
                        self.tiles[other.row][other.column] = Tile::Explored;
                    }
                }
            }
            current_frontier = next_frontier;
            next_frontier = HashSet::new();
            println!("\n\nStep {}", step);
            self.print();
            for (to_restore, tile) in restore {
                self.tiles[to_restore.row][to_restore.column] = tile;
            }
        }

        return total_others.len();
    }
}

#[test]
fn flooding() {
    let mut field = get_test_input();
    assert_eq!(16, field.flood(6));
}
