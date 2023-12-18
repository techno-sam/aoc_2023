use std::fs;

use char_enum_impl::{char_enum, data_enum};
use utils::{colorize, highlight};

fn main() {
    println!("AOC 2023 Day 16");

    let contents = fs::read_to_string("src/bin/day16/input.txt").expect("Failed to read input");

    let mut field: Field;
    let test = false;
    if !test {
        field = Field::parse(&contents, 0, 0, BeamDirection::East);
    } else {
        field = get_test();
    }

    if test {
        field.print();
    }

    for step in 1_usize..=2000 {
        if field.step() {
            println!("Exiting early on step {} because beams are done", step);
            break;
        }
        field.cleanup_outer();

        if step % 10 == 0 && test {
            println!("\nStep {}", step);
            field.print();
        }
    }
    field.cleanup_outer();

    if test {
        println!("\nFinal");
        field.print();
    }

    println!("\n\nPart 1 final count: {}", field.count());

    // subtract out padding
    let width = field.width - 2;
    let height = field.height - 2;

    let mut max = 0;

    println!("Iterating through {} starting locations", (width*2) + (height*2));

    // top going down and bottom going up
    for column in 0..width {
        let mut field = Field::parse(&contents, 0, column, BeamDirection::South);
        while !field.step() {}
        field.cleanup_outer();
        max = max.max(field.count());

        let mut field = Field::parse(&contents, height-1, column, BeamDirection::North);
        while !field.step() {}
        field.cleanup_outer();
        max = max.max(field.count());
    }

    println!("Done with top and bottom entry");

    // left going east and right going west
    for row in 0..height {
        let mut field = Field::parse(&contents, row, 0, BeamDirection::East);
        while !field.step() {}
        field.cleanup_outer();
        max = max.max(field.count());

        let mut field = Field::parse(&contents, row, width-1, BeamDirection::West);
        while !field.step() {}
        field.cleanup_outer();
        max = max.max(field.count());
    }

    println!("\n\nPart 2 final count: {}", max);
}

#[test]
fn end_to_end() {
    let mut field = get_test();
    while !field.step() {}
    field.cleanup_outer();
    assert_eq!(46, field.count());
}

fn get_test() -> Field {
    return Field::parse(r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
", 0, 0, BeamDirection::East);
}

#[char_enum]
enum Tile {
    Outer = '#',
    Empty = '.',
    MirrorForwards = '/',
    MirrorBackwards = '\\',
    SplitterVert = '|',
    SplitterHorz = '-'
}

#[derive(Copy, Clone)]
#[data_enum((isize, isize))]
enum BeamDirection {
    North = (0, -1),
    South = (0, 1),
    East = (1, 0),
    West = (-1, 0)
}
impl BeamDirection {
    fn is_vertical(&self) -> bool {
        match self {
            BeamDirection::North | BeamDirection::South => true,
            _ => false
        }
    }

    fn is_horizontal(&self) -> bool {
        match self {
            BeamDirection::East | BeamDirection::West => true,
            _ => false
        }
    }

    /// Reflect along [Tile::MirrorForwards]
    fn reflect_forward(self) -> BeamDirection {
        match self {
            BeamDirection::North => BeamDirection::East,
            BeamDirection::East => BeamDirection::North,

            BeamDirection::South => BeamDirection::West,
            BeamDirection::West => BeamDirection::South
        }
    }

    /// Reflect along [Tile::MirrorBackwards]
    fn reflect_backward(self) -> BeamDirection {
        match self {
            BeamDirection::North => BeamDirection::West,
            BeamDirection::West => BeamDirection::North,

            BeamDirection::South => BeamDirection::East,
            BeamDirection::East => BeamDirection::South
        }
    }

    fn bitmap_val(&self) -> u8 {
        match self {
            BeamDirection::North => 0b0001,
            BeamDirection::South => 0b0010,
            BeamDirection::East  => 0b0100,
            BeamDirection::West  => 0b1000
        }
    }
}

struct Beam {
    direction: BeamDirection,
    row: usize,
    column: usize
}
impl Beam {
    fn offset(&mut self, direction: BeamDirection) {
        self.direction = direction;
        let (ocol, orow) = self.direction.value();
        self.row = ((self.row as isize) + orow) as usize;
        self.column = ((self.column as isize) + ocol) as usize;
    }

    fn clone(&self) -> Beam {
        return Beam { direction: self.direction, row: self.row, column: self.column };
    }

    #[inline]
    fn prev_row(&self) -> usize {
        let (_, orow) = self.direction.value();
        ((self.row as isize) - orow) as usize
    }

    #[inline]
    fn prev_column(&self) -> usize {
        let (ocol, _) = self.direction.value();
        ((self.column as isize) - ocol) as usize
    }
}

struct Field {
    /// [row][column] indexed
    tiles: Vec<Vec<Tile>>,
    /// [row][column] indexed
    lit: Vec<Vec<bool>>,

    /// [row][column] indexed with data about where beams have already travelled
    done: Vec<Vec<u8>>,

    width: usize,
    height: usize,
    beams: Vec<Beam>
}
impl Field {
    fn parse(data: &str, start_row: usize, start_column: usize, start_dir: BeamDirection) -> Field {
        let mut lines: Vec<String> = data.trim().split("\n").map(|l| "#".to_owned() + l + "#").collect();

        let width = lines[0].len();
        lines.insert(0, "#".repeat(width));
        lines.push("#".repeat(width));
        let height = lines.len();

        let lit: Vec<Vec<bool>> = {
            let mut lit = vec![vec![false; width]; height];
            // the + 1 is to account for padding
            lit[start_row+1][start_column+1] = true;
            lit
        };
        let done: Vec<Vec<u8>> = vec![vec![0; width]; height];
        let tiles = lines.into_iter()
            .map(|s| s.chars()
                .map(|c| Tile::decode(c))
                .collect()
            )
            .collect();

        let beams = vec![Beam { direction: start_dir, row: start_row+1, column: start_column+1}];

        return Field { tiles, lit, width, height, beams, done };
    }

    /// should be called POST travel and before [Field::mark_done]
    #[inline]
    fn is_done(&self, beam: &Beam) -> bool {
        let data = self.done[beam.row][beam.column];
        return (data & beam.direction.bitmap_val()) != 0;
    }

    /// should be called POST travel
    #[inline]
    fn mark_done(&mut self, beam: &Beam) {
        self.done[beam.prev_row()][beam.prev_column()] |= beam.direction.bitmap_val();
    }

    fn print(&self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let (r, g, b) = if self.lit[row][column] { (0, 255, 0) } else { (150, 0, 0) };
                let colorized = colorize(&format!("{}", self.tiles[row][column].encode()), r, g, b);
                let actually = {
                    let mut actually: bool = false;
                    for beam in &self.beams {
                        if beam.row == row && beam.column == column {
                            actually = true;
                            break;
                        }
                    }
                    actually
                };
                let highlighted = highlight(&colorized, actually, 0, 120, 120);
                print!("{}", highlighted);
            }
            println!("");
        }
    }

    /// Move every beam one step
    /// return: done (no more beams)
    fn step(&mut self) -> bool {
        if self.beams.len() == 0 {
            return true;
        }
        let mut to_remove: Vec<usize> = vec![];
        let mut to_add: Vec<Beam> = vec![];
        for idx in 0..self.beams.len() {
            let beam = &mut self.beams[idx];
            // going to have to do lots of mutable-immutable re-definitions to get this all to work
            let current = &self.tiles[beam.row][beam.column];
            match current {
                Tile::Outer => {
                    self.lit[beam.row][beam.column] = false;
                    to_remove.push(idx);
                },
                Tile::Empty => {
                    beam.offset(beam.direction);
                    self.lit[beam.row][beam.column] = true;
                    let beam = &self.beams[idx].clone(); // drop the mutable reference

                    if self.is_done(beam) {
                        to_remove.push(idx);
                    } else {
                        self.mark_done(beam);
                    }
                },
                Tile::SplitterVert => {
                    if beam.direction.is_horizontal() {
                        let mut new = beam.clone();
                        beam.offset(BeamDirection::North);
                        new.offset(BeamDirection::South);

                        self.lit[beam.row][beam.column] = true;
                        self.lit[new.row][new.column] = true;
                        to_add.push(new);
                    } else {
                        beam.offset(beam.direction);
                        self.lit[beam.row][beam.column] = true;
                    }
                },
                Tile::SplitterHorz => {
                    if beam.direction.is_vertical() {
                        let mut new = beam.clone();
                        beam.offset(BeamDirection::East);
                        new.offset(BeamDirection::West);

                        self.lit[beam.row][beam.column] = true;
                        self.lit[new.row][new.column] = true;
                        to_add.push(new);
                    } else {
                        beam.offset(beam.direction);
                        self.lit[beam.row][beam.column] = true;
                    }
                },
                Tile::MirrorForwards => {
                    beam.offset(beam.direction.reflect_forward());
                    self.lit[beam.row][beam.column] = true;
                },
                Tile::MirrorBackwards => {
                    beam.offset(beam.direction.reflect_backward());
                    self.lit[beam.row][beam.column] = true;
                }
            };
        }
        to_remove.sort();
        let mut offset = 0;
        // since we remove from start to end, it is essential that we subtract 1 from the target
        // index for each beam we remove
        for remove_idx in to_remove {
            self.beams.remove(remove_idx - offset);
            offset += 1;
        }
        self.beams.extend(to_add);
        return false;
    }

    /// Delete and 'unlight' beams in outer cells
    fn cleanup_outer(&mut self) {
        let mut to_remove: Vec<usize> = vec![];
        
        for (idx, beam) in self.beams.iter().enumerate() {
            let current = &self.tiles[beam.row][beam.column];
            if let Tile::Outer = current {
                self.lit[beam.row][beam.column] = false;
                to_remove.push(idx);
            }
        }

        to_remove.sort();
        let mut offset = 0;
        // since we remove from start to end, it is essential that we subtract 1 from the target
        // index for each beam we remove
        for remove_idx in to_remove {
            self.beams.remove(remove_idx - offset);
            offset += 1;
        }
    }

    fn count(&self) -> usize {
        let mut count = 0;
        for row in 0..self.height {
            for column in 0..self.width {
                if self.lit[row][column] {
                    count += 1;
                }
            }
        }
        return count;
    }
}
