use std::{fs, hash::{Hash, Hasher}};

use memoize::memoize;

fn main() {
    println!("AOC 2023 Day 14");

    let contents = fs::read_to_string("src/bin/day14/input.txt").expect("Failed to read input");
    let platform = Platform::parse(&contents);
    println!("Part 1: {}", platform.load());

    println!("Spinning a billion times");
    let load2 = platform.a_billion_cycles().rotate_no_tilt().load();
    println!("Part 2: {}", load2);
/*
    println!("\n\n");
    debugging();*/
}

#[allow(dead_code)]
fn debugging() {
    let platform = Platform::parse("
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
").initial_spin_cycle();
    println!("After 1 cycle:");
    platform.rotate_no_tilt().print();
}

#[derive(Clone, Copy, PartialEq, Hash)]
enum Tile {
    RoundRock,
    CubeRock,
    Empty
}
impl Tile {
    fn decode(chr: char) -> Tile {
        match chr {
            'O' => Tile::RoundRock,
            '#' => Tile::CubeRock,
            '.' => Tile::Empty,
            _ => panic!("Invalid tile")
        }
    }

    fn encode(&self) -> char {
        match self {
            Tile::RoundRock => 'O',
            Tile::CubeRock => '#',
            Tile::Empty => '.'
        }
    }
}

#[memoize]
fn spin_cycle(platform: Platform) -> Platform {
    let mut p: Platform = platform;
    for _ in 1..=4 {
        p = p.spin();
    }
    return p;
}

#[inline(always)]
fn greatest_multiple(v: usize, less_than: usize) -> usize {
    println!("Calculation greatest multiple of {} less than {}", v, less_than);
    let count: usize = less_than / v;
    return v * count;
}

#[derive(Clone)]
struct Platform {
    /// columns[0] is left, columns[#len-1] is right
    /// columns[0][0] is north, columns[0][#len-1] is south
    columns: Vec<Vec<Tile>>,
    southern_space: Vec<usize>, // index of the southern-most supported space (empty itself)
    height: usize
}
impl Platform {
    fn a_billion_cycles(self) -> Platform {
        // cycle length seems to be 7 (aka can skip 7n steps for same result)
        //const CYCLES: usize = 6 + 70;
        const CYCLES: usize = 1_000_000_000;
        let mut old: Vec<Platform> = vec![self.clone()];
        let mut p: Platform = self;
        let mut skip: Option<usize> = None;
        for i in 0..CYCLES {
            if let Some(s) = skip {
                if s > 0 {
                    skip = Some(s -1);
                    continue;
                }
            }
            if i == 0 {
                p = p.initial_spin_cycle();

                #[cfg(test)]
                println!("\nAfter initial cycle:");
                #[cfg(test)]
                p.clone().rotate_no_tilt().print();
            } else {
                p = p.spin_cycle();
                #[cfg(test)]
                if i < 3 {
                    println!("\nAfter {} cycles", i+1);
                    p.clone().rotate_no_tilt().print();
                }
            }
            
            if let None = skip {
                for (loop_start, o) in (&old).iter().enumerate() {
                    if p == *o {
                        let cycle_len = i-loop_start + 1;
                        println!("i={}, LOOP starting at {}, with len {}", i, loop_start, cycle_len);
                        let s = greatest_multiple(cycle_len, CYCLES - i);
                        println!("Skipping {}", s);

                        #[cfg(test)]
                        println!("ME");
                        #[cfg(test)]
                        p.print();
                        #[cfg(test)]
                        println!("OTHER");
                        #[cfg(test)]
                        o.print();

                        skip = Some(s);
                    }
                }
                old.push(p.clone());
            }
        }
        return p;
    }

    fn print(&self) {
        for i in 0..self.height {
            println!("{}", String::from_iter(self.columns.iter().map(|c| format!("{}", c[i].encode()))));
        }
    }

    #[inline(always)]
    fn spin_cycle(self) -> Platform {
        return spin_cycle(self);
    }

    fn initial_spin_cycle(self) -> Platform {
        #[cfg(test)]
        println!("\nbefore spin:");
        #[cfg(test)]
        self.print();

        return self.spin().spin().spin(); // only 3 spins, since we are initialized with a north
                                          // tilt
    }

    fn spin(&self) -> Platform {
        let mut columns = vec![];
        let mut southern_space = vec![];
        for _ in 0..self.height {
            columns.push(vec![]);
            southern_space.push(0);
        }
        let height = self.columns.len();
        let mut platform = Platform { columns, southern_space, height };
        for column in &self.columns {
            platform.insert(column.iter().rev().map(|t| *t).collect());
        }
        /*println!("\nSPUN");
        platform.print();*/
        return platform;
    }

    fn rotate_no_tilt(&self) -> Platform {
        let mut columns = vec![];
        let mut southern_space = vec![];
        for _ in 0..self.height {
            columns.push(vec![]);
            southern_space.push(0);
        }
        let height = self.columns.len();
        let mut platform = Platform { columns, southern_space, height };
        for column in &self.columns {
            let row = column.iter().rev().map(|t| *t);
            for (i, tile) in row.enumerate() {
                platform.columns[i].push(tile);
            }
        }
        return platform;
    }

    fn parse(data: &str) -> Platform {
        let lines: Vec<&str> = data.trim().split("\n").collect();
        let width = lines[0].len();
        let mut columns = vec![];
        let mut southern_space = vec![];
        for _ in 0..width {
            columns.push(vec![]);
            southern_space.push(0);
        }

        let mut platform = Platform { columns, southern_space, height: lines.len() };
        for line in lines {
            let tiles = line.chars().map(|c| Tile::decode(c)).collect();
            platform.insert(tiles);
        }
        return platform;
    }

    fn insert(&mut self, row: Vec<Tile>) {
        assert_eq!(self.columns.len(), row.len());
        for (i, tile) in row.into_iter().enumerate() {
            match tile {
                Tile::RoundRock => {
                    self.columns[i].push(Tile::Empty); // if there's a supporting block right
                                                       // below, then this will be immediately
                                                       // overriden by a RoundRock
                    self.columns[i][self.southern_space[i]] = Tile::RoundRock;
                    self.southern_space[i] += 1;
                },
                Tile::CubeRock => {
                    self.columns[i].push(Tile::CubeRock);
                    self.southern_space[i] = self.columns[i].len();
                },
                Tile::Empty => self.columns[i].push(Tile::Empty)
            }
        }
    }

    fn load(&self) -> usize {
        let mut load = 0;
        for i in 0..self.height {
            let factor = self.height - i;
            for column in &self.columns {
                if let Tile::RoundRock = column[i] {
                    load += factor;
                }
            }
        }
        return load;
    }
}
impl PartialEq for Platform {
    fn eq(&self, other: &Platform) -> bool {
        if self.height != other.height {
            return false;
        }
        if self.columns != other.columns {
            return false;
        }
        return true;
    }
}
impl Eq for Platform {}
impl Hash for Platform {
    fn hash<H>(&self, state: &mut H) where H: Hasher {
        self.columns.hash(state);
    }
}

#[test]
fn load_calculation() {
    assert_eq!(136, Platform::parse("
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
").load());
}

#[test]
fn spin_load_calculation() {
    assert_eq!(87, Platform::parse("
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
").initial_spin_cycle().rotate_no_tilt().load());
}

#[test]
#[ignore = "Slow, takes about 7 seconds"]
fn all_spin_load_calculation() {
    assert_eq!(64, Platform::parse("
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
").a_billion_cycles().rotate_no_tilt().load());
}
