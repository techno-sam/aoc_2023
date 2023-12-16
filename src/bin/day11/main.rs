use std::fs;

fn main() {
    println!("AOC 2023 Day 11");

    let contents = fs::read_to_string("src/bin/day11/input.txt").expect("Failed to read input");
    let map: &mut StarMap = &mut StarMap::load(&contents);

    map.expand_all();

    let dist = map.sum_distances(1);
    println!("Distance sum: {}", dist);
    let dist2 = map.sum_distances(1_000_000-1);
    println!("Long-distance sum: {}", dist2);
}

#[allow(dead_code)]
fn get_test_map() -> StarMap {
    return StarMap::load("
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
");
}

struct Coord {
    x: u64, // column
    y: u64  // row
}
#[allow(dead_code)]
impl Coord {
    #[inline]
    fn column(&self) -> u64 {
        return self.x;
    }

    #[inline]
    fn row(&self) -> u64 {
        return self.y;
    }

    fn dist(&self, other: &Coord, expanded_rows: &Vec<usize>, expanded_columns: &Vec<usize>, expansion: u64) -> u64 {
        let x1 = self.x.min(other.x);
        let x2 = self.x.max(other.x);

        let y1 = self.y.min(other.y);
        let y2 = self.y.max(other.y);

        let mut dist = (x2-x1) + (y2-y1);

        for row in expanded_rows {
            let r = *row as u64;
            if r > y1 && r < y2 {
                dist += expansion;
            }
        }
        for column in expanded_columns {
            let c = *column as u64;
            if c > x1 && c < x2 {
                dist += expansion;
            }
        }

        return dist;
    }
}

enum StarType {
    EmptySpace,
    Galaxy
}
impl StarType {
    fn decode(chr: char) -> Option<StarType> {
        return match chr {
            '.' => Some(StarType::EmptySpace),
            '#' => Some(StarType::Galaxy),
            _ => None
        };
    }

    fn encode(&self) -> char {
        return match self {
            StarType::EmptySpace => '.',
            StarType::Galaxy => '#',
        }
    }
}

struct StarMap {
    field: Vec<Vec<StarType>>,
    width: usize,
    height: usize,
    expanded_rows: Vec<usize>,
    expanded_columns: Vec<usize>
}
impl StarMap {
    fn galaxy_coords(&self) -> Vec<Coord> {
        let mut coords: Vec<Coord> = vec![];
        for row in 0..self.height {
            for column in 0..self.width {
                if let StarType::Galaxy = self.field[row][column] {
                    coords.push(Coord { x: column as u64, y: row as u64 });
                }
            }
        }
        return coords;
    }

    fn sum_distances(&self, expansion: u64) -> u64 {
        let mut sum = 0;
        let coords = self.galaxy_coords();

        for i in 0..coords.len() {
            let c1 = &coords[i];
            for j in i+1..coords.len() {
                let c2 = &coords[j];
                sum += c1.dist(c2, &self.expanded_rows, &self.expanded_columns, expansion);
            }
        }

        return sum;
    }

    fn load(data: &str) -> StarMap {
        let mut field: Vec<Vec<StarType>> = vec![];

        let lines = data.trim().split("\n").enumerate();
        let points = lines.map(|(i, l)| (i, l.trim().chars().enumerate()));
        for (_, line) in points {
            let mut row: Vec<StarType> = vec![];
            for (_, chr) in line {
                row.push(StarType::decode(chr).expect(&format!("Unkown character {}", chr)));
            }
            field.push(row);
        }

        let width = field[0].len();
        let height = field.len();
        return StarMap { field, width, height, expanded_rows: vec![], expanded_columns: vec![] };
    }

    fn expand_all(&mut self) {
        self.expand_empty_columns();
        self.expand_empty_rows();
    }

    fn expand_empty_columns(&mut self) {
        let mut empty: Vec<bool> = Vec::with_capacity(self.width);
        for _ in 0..self.width {
            empty.push(true);
        }

        for row in 0..self.height {
            let line = &self.field[row];
            for column in 0..self.width {
                let st = &line[column];
                if let StarType::EmptySpace = st {
                } else {
                    empty[column] = false;
                }
            }
        }

        //let mut offset: usize = 0;
        let to_expand: Vec<usize> = empty.iter().enumerate().filter(|(_, &e)| e).map(|(i, _)| i).collect();
        self.expanded_columns = to_expand;

        /*for expand_column in to_expand {
            offset += 1;
            for line in &mut self.field {
                line.insert(expand_column + offset, StarType::EmptySpace);
            }
        }
        self.width += offset;*/
    }

    fn expand_empty_rows(&mut self) {
        let mut to_expand: Vec<usize> = vec![];
        for row in 0..self.height {
            let empty = &self.field[row].iter().all(|v| {
                if let StarType::EmptySpace = v {
                    true
                } else {
                    false
                }
            });

            if *empty {
                to_expand.push(row);
            }
        }

        self.expanded_rows = to_expand;
        /*let mut offset: usize = 0;
        for expand_row in to_expand {
            offset += 1;
            let mut new_row: Vec<StarType> = vec![];
            for _ in 0..self.width {
                new_row.push(StarType::EmptySpace);
            }
            self.field.insert(expand_row+offset, new_row);
        }
        self.height += offset;*/
    }

    #[allow(dead_code)]
    fn print(&self) {
        for line in &self.field {
            for st in line {
                print!("{}", st.encode());
            }
            println!("");
        }
    }
}

#[test]
fn counting() {
    let map: &mut StarMap = &mut StarMap::load("
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
");
    map.expand_empty_columns();
    map.expand_empty_rows();

    assert_eq!(374, map.sum_distances(1));
    assert_eq!(1030, map.sum_distances(9));
    assert_eq!(8410, map.sum_distances(99));
}
