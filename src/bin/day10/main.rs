use std::{fs, collections::hash_map::DefaultHasher, hash::{Hash, Hasher}, time::SystemTime};

fn main() {
    println!("AOC 2023 Day 10");

    let test = true;
    let mut map: Map = if test {parse_example(2, true)} else {parse_input()};

    println!("Map is {}x{}", map.width, map.height);

    map.build_all();
    println!("Starting coord: {:?}", map.starting_pos);
    map.calculate_starting_network();

    map.print();
    println!("");

    let max_dist = map.calculate_distance();
    if test {
        map.print_dist();
    }

    println!("\nmax dist: {}", max_dist);
}

#[derive(PartialEq, Clone, Copy)]
enum Side {
    North,
    East,
    South,
    West
}
impl Side {
    fn opposite(&self) -> Side {
        return match self {
            Side::North => Side::South,
            Side::South => Side::North,
            Side::East  => Side::West,
            Side::West  => Side::East
        };
    }

    fn offset(&self, coord: &Coord) -> Coord {
        let row_offset: i64 = match self {
            Side::North => -1,
            Side::South =>  1,
            _ => 0
        };
        let column_offset: i64 = match self {
            Side::West => -1,
            Side::East =>  1,
            _ => 0
        };
        return Coord { row: (coord.row as i64 + row_offset) as usize, column: (coord.column as i64 + column_offset) as usize };
    }

    fn values() -> Vec<Side> {
        return vec![Side::North, Side::East, Side::South, Side::West];
    }
}

#[derive(PartialEq, Debug)]
enum Pipe {
    NS,     // |
    EW,     // -
    NE,     // L
    NW,     // J
    SW,     // 7
    SE,     // F
    Ground, // .
    Start   // S
}
impl Pipe {
    fn parse(data: char) -> Option<Pipe> {
        return match data {
            '|' => Some(Pipe::NS),
            '-' => Some(Pipe::EW),
            'L' => Some(Pipe::NE),
            'J' => Some(Pipe::NW),
            '7' => Some(Pipe::SW),
            'F' => Some(Pipe::SE),
            '.' => Some(Pipe::Ground),
            'S' => Some(Pipe::Start),
            _ => None
        }
    }

    fn to_pretty(&self) -> String {
        return match self {
            Pipe::NS => "│",
            Pipe::EW => "─",
            Pipe::NE => "└",
            Pipe::NW => "┘",
            Pipe::SW => "┐",
            Pipe::SE => "┌",
            Pipe::Ground => ".",
            Pipe::Start => "◎",
        }.to_string();
    }

    fn is_side_open(&self, side: &Side) -> bool {
        return match self {
            Pipe::NS => *side == Side::North || *side == Side::South,
            Pipe::EW => *side == Side::East  || *side == Side::West,
            Pipe::NE => *side == Side::North || *side == Side::East,
            Pipe::NW => *side == Side::North || *side == Side::West,
            Pipe::SW => *side == Side::South || *side == Side::West,
            Pipe::SE => *side == Side::South || *side == Side::East,
            _ => false
        };
    }
}

struct Entry {
    pipe: Pipe,
    id: Option<u64>,
    distance: u64,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Coord {
    row: usize,
    column: usize
}

struct Map {
    /// [row][column] indexed
    pipes: Vec<Vec<Entry>>,
    starting_pos: Coord,
    width: usize,
    height: usize,
    max_id: u64
}
impl Map {
    fn parse(data: &str) -> Map {
        let mut starting_pos: Option<Coord> = None;

        let mut lines: Vec<&str> = data.trim().split("\n").collect();
        let height: usize = lines.len() + 2;
        let width: usize = lines[0].chars().count() + 2;
        
        let pad = ".".repeat(width);
        lines.insert(0, &pad);
        lines.push(&pad);

        let pipes = lines.iter()
            .map(|line| (".".to_owned()+line+".").chars()
                .map(|c| Pipe::parse(c).expect("Failed to parse pipe"))
                .map(|p| Entry { pipe: p, id: None, distance: 0xffff_ffff_ffff_ffff })
                .collect::<Vec<_>>()
            )
            .collect::<Vec<Vec<Entry>>>();
        'Outer: for row in 0..height {
            for column in 0..width {
                if pipes[row][column].pipe == Pipe::Start {
                    starting_pos = Some(Coord { row, column });
                    break 'Outer;
                }
            }
        }
        if let Some(sp) = starting_pos {
            let time: u64 = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs(); 
            return Map { pipes, starting_pos: sp, width, height, max_id: time};
        }
        panic!("No starting point found");
    }
    
    fn print(&self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let entry = &self.pipes[row][column];
                print!("{}", highlight(&colorize_id(&entry.pipe.to_pretty(), entry.id.unwrap_or(0)), entry.pipe == Pipe::Start));
            }
            println!("");
        }
    }

    fn print_dist(&self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let entry = &self.pipes[row][column];
                print!("{} ", colorize_id(&entry.distance.to_string(), entry.id.unwrap_or(0)));
            }
            println!("");
        }
    }

    fn build_network(&mut self, start_column: usize, start_row: usize) {
        let id = self.max_id;
        self.max_id += 1;

        let mut frontier: Vec<Coord> = vec![Coord { row: start_row, column: start_column }];
        while !frontier.is_empty() {
            let c = &frontier.pop().unwrap();
            let pipe = &mut self.pipes[c.row][c.column];
            if let Some(_) = pipe.id {
                continue;
            }
            pipe.id = Some(id);
            // don't need to mutate anymore
            let pipe = &self.pipes[c.row][c.column];

            for side in &Side::values() {
                if pipe.pipe.is_side_open(side) {
                    let other_c = side.offset(c);
                    if self.pipes[other_c.row][other_c.column].pipe.is_side_open(&side.opposite()) {
                        frontier.push(other_c);
                    }
                }
            }
        }
    }

    fn build_all(&mut self) {
        for row in 1..self.height-1 {
            for column in 1..self.width-1 {
                self.build_network(column, row);
            }
        }
    }

    fn calculate_starting_network(&mut self) {
        let start = &mut self.pipes[self.starting_pos.row][self.starting_pos.column];
        start.id = None;
        assert_eq!(Pipe::Start, start.pipe);

        let networks: Vec<u64> = Side::values().iter()
            .map(|s| s.offset(&self.starting_pos))
            .map(|c| self.pipes[c.row][c.column].id)
            .filter(|o| if let Some(_) = o {true} else {false})
            .map(|o| o.unwrap())
            .collect();
        for id in &networks {
            let mut count = 0;
            for other in &networks {
                if *other == *id {
                    count += 1;
                }
            }
            if count >= 2 {
                self.pipes[self.starting_pos.row][self.starting_pos.column].id = Some(*id);
                return;
            }
        };
    }

    fn get(&self, c: Coord) -> &Entry {
        return &self.pipes[c.row][c.column];
    }

    fn get_mut(&mut self, c: Coord) -> &mut Entry {
        return &mut self.pipes[c.row][c.column];
    }

    fn calculate_distance(&mut self) -> u64 {
        for row in 0..self.height {
            for column in 0..self.width {
                self.pipes[row][column].distance = 0xffff_ffff_ffff_ffff;
            }
        }
        let mut max = 0;
        let mut frontier: Vec<Coord> = vec![self.starting_pos];
        let mut visited: Vec<Coord> = vec![self.starting_pos];
        self.get_mut(self.starting_pos).distance = 0;
        let main_id = self.get(self.starting_pos).id.expect("No starting id");
        while !frontier.is_empty() {
            let c = frontier.pop().unwrap();
            if c == self.starting_pos {
                Side::values().iter()
                    .map(|s| (s, s.offset(&c)))
                    .filter(|(s, coord)| self.get(*coord).pipe.is_side_open(&s.opposite())
                        && self.get(*coord).id.unwrap_or(0) == main_id)
                    .for_each(|(_, coord)| frontier.insert(0, coord));
                continue;
            }
            if visited.contains(&c) {
                continue;
            }
            visited.push(c);
            let entry = self.get(c);

            let connected: Vec<Side> = Side::values().iter()
                .filter(|s| entry.pipe.is_side_open(s))
                .map(|s| *s)
                .collect();

            let dist = connected.iter().map(|s| s.offset(&c)).map(|c| self.get(c).distance).min().unwrap() + 1;
            self.get_mut(c).distance = dist;
            if dist > max {
                max = dist;
            }
            connected.iter().map(|s| s.offset(&c)).for_each(|c| frontier.insert(0, c));
        }

        let main_id = self.get(self.starting_pos).id.unwrap();
        for row in 0..self.height {
            for column in 0..self.width {
                if self.pipes[row][column].id.unwrap_or(0) != main_id {
                    self.pipes[row][column].distance = 0;
                }
            }
        }
        return max;
    }
}

fn parse_example(id: usize, extra_pipes: bool) -> Map {
    let contents = fs::read_to_string("src/bin/day10/ex0".to_owned()+&id.to_string()+(if extra_pipes {"b"} else {"a"})+".txt")
        .expect("Failed to load exmaple");
    return Map::parse(&contents);
}

fn parse_input() -> Map {
    let contents = fs::read_to_string("src/bin/day10/input.txt").expect("Failed to load input");
    return Map::parse(&contents);
}

fn colorize_id(input: &str, id: u64) -> String {
    if id == 0 {
        return input.to_owned();
    }
    let mut hasher = DefaultHasher::new();
    id.hash(&mut hasher);
    let out = hasher.finish();
    let (r, g, b) = rainbow_color(out);
    return colorize(input, r, g, b);
}

// rainbow-y stuff taken from https://github.com/Creators-of-Create/Create under the MIT license
fn color_in_phase(phase: u64, progress: u8) -> u8 {
    let p = phase % 6;
    if p <= 1 {
        return 0;
    } else if p == 2 {
        return progress;
    } else if p <= 4 {
        return 255;
    } else {
        return 255 - progress;
    }
}

fn rainbow_color(time_step: u64) -> (u8, u8, u8) {
    let loc_time_step = time_step % 1536;
    let time_step_in_phase = (loc_time_step % 256) as u8;
    let phase_blue = loc_time_step / 256;
    let red = color_in_phase(phase_blue + 4, time_step_in_phase);
    let green = color_in_phase(phase_blue + 2, time_step_in_phase);
    let blue = color_in_phase(phase_blue, time_step_in_phase);
    return (red, green, blue);
}
// end Create rainbowy stuff

fn colorize(input: &str, r: u8, g: u8, b: u8) -> String {
    return "\x1b[38;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

fn highlight(input: &str, actually: bool) -> String {
    if !actually {
        return input.to_owned();
    }
    return "\x1b[42m".to_owned()+input+"\x1b[0m";
}
