use std::fs;

fn main() {
    println!("AOC 2023 Day 13");

    let contents = fs::read_to_string("src/bin/day13/input.txt").expect("Failed to read input");
    let sum = calculate(&contents);
    println!("Part 1: {}", sum);
}

fn calculate(contents: &str) -> usize {
    let fields = contents.trim().split("\n\n").map(|f| AshField::load(f));
    let mut sum = 0;
    for (i, field) in fields.enumerate() {
        match field.get_vertical_mirror() {
            Some(v) => sum += v,
            None => println!("No vertical mirror in field {}", i)
        }
        match field.get_horizontal_mirror() {
            Some(v) => sum += 100*v,
            None => println!("No horizontal mirror in field {}", i)
        }
        println!("");
        field.visualize();
    }
    return sum;
}

struct AshField {
    rows: Vec<String>,
    columns: Vec<String>,
    width: usize,
    height: usize
}
impl AshField {
    fn load(data: &str) -> AshField {
        let rows: Vec<String> = data.trim().split("\n").map(|s| s.to_owned()).collect();
        let mut columns: Vec<String> = vec![];
        for column in 0..rows[0].len() {
            let mut c = "".to_owned();
            for row in &rows {
                c += &format!("{}", row.chars().nth(column).unwrap());
            }
            columns.push(c);
        }
        let width = columns.len();
        let height = rows.len();
        return AshField { rows, columns, width, height };
    }

    fn get_horizontal_mirror(&self) -> Option<usize> {
        println!("\n\nchecking mirror with height {}", self.height);
        // Strategy:
        // Iterate through the rows and if rows[i] == rows[i+1], expand out from there to check
        'Outer: for i in 0..self.height-1 {
            if self.rows[i] == self.rows[i+1] {
                println!("\n> found potential mirroring at {} <-> {}", i, i+1);
                // check outwards
                let max_offset = i.min(self.height-i-2);
                println!("> min checked: {}, max checked: {}", i-max_offset, i+max_offset);
                for offset in 1..=max_offset {
                    if self.rows[i-offset] != self.rows[i+offset+1] {
                        println!(">> failed cmp for {} <-> {}", i-offset, i+offset+1);
                        continue 'Outer;
                    }
                }
                return Some(i+1);
            }
        }
        return None;
    }

    fn get_vertical_mirror(&self) -> Option<usize> {
        // Strategy:
        // Iterate through the columns and if columns[i] == columns[i+1], expand out from there to check
        'Outer: for i in 0..self.width-1 {
            if self.columns[i] == self.columns[i+1] {
                // check outwards
                let max_offset = i.min(self.width-i-2);
                for offset in 1..=max_offset {
                    if self.columns[i-offset] != self.columns[i+offset+1] {
                        continue 'Outer;
                    }
                }
                return Some(i+1);
            }
        }
        return None;
    }

    fn visualize(&self) {
        match self.get_horizontal_mirror() {
            Some(v) => {
                for i in 0..self.height {
                    let (r, g, b) = if i < v { (0, 255, 0) } else { (255, 0, 0) };
                    println!("{}", colorize(&self.rows[i], r, g, b));
                }
                return;
            },
            None => {}
        }

        match self.get_vertical_mirror() {
            Some(v) => {
                for line in &self.rows {
                    let colorized = line.chars().enumerate().map(|(i, c)| {
                        let (r, g, b) = if i < v { (0, 255, 0) } else { (255, 0, 0) };
                        colorize(&format!("{}", c), r, g, b)
                    }).collect::<Vec<String>>().join("");
                    println!("{}", colorized);
                }
                return;
            },
            None => {}
        }

        for line in &self.rows {
            println!("{}", line);
        }
    }
}

#[allow(dead_code)]
fn colorize(input: &str, r: u8, g: u8, b: u8) -> String {
    return "\x1b[38;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

#[test]
fn horizontal_detection() {
    assert_eq!(None, AshField::load("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.").get_horizontal_mirror(), "0 lacks horizontal");
    assert_eq!(Some(4), AshField::load("
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#").get_horizontal_mirror(), "1 has horizontal(4)");
}

#[test]
fn vertical_detection() {
    assert_eq!(Some(5), AshField::load("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.").get_vertical_mirror(), "0 has vertical(5)");
    assert_eq!(None, AshField::load("
#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#").get_vertical_mirror(), "1 lacks vertical");
}

#[test]
fn end_to_end() {
    assert_eq!(405, calculate("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"));
    assert_eq!(709, calculate("
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#

.#.##.#.#
.##..##..
.#.##.#..
#......##
#......##
.#.##.#..
.##..##.#

#..#....#
###..##..
.##.#####
.##.#####
###..##..
#..#....#
#..##...#

#.##..##.
..#.##.#.
##..#...#
##...#..#
..#.##.#.
..##..##.
#.#.##.#.
"));
}
