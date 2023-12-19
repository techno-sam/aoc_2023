use std::{fs, collections::HashSet};

use char_enum_impl::char_enum;

fn main() {
    println!("AOC 2023 Day 18");

    let contents: String;
    if true {
        contents = fs::read_to_string("src/bin/day18/input.txt").expect("Failed to read input");
    } else {
        contents = example();
    }
    let data: Vec<(Moves, u8, &str)> = contents.trim().split("\n")
        .map(|line| line
            .strip_suffix(")").unwrap()
            .split_once(" (").unwrap()
        ).map(|(mov, color)| (mov.split_once(" ").unwrap(), color))
        .map(|((mov, dist), color)|
            (
                Moves::decode(mov.chars().last().unwrap()),
                dist.parse::<u8>().unwrap(),
                color
            )
        )
        .collect();

    let mut field: HashSet<(isize, isize)> = HashSet::new();

    let (mut x, mut y) = (0, 0);
    field.insert((x, y));

    let mut min_x = 0;
    let mut max_x = 0;

    let mut min_y = 0;
    let mut max_y = 0;

    for (mov, dist, _) in data {
        for _ in 0..dist {
            (x, y) = mov.offset(x, y);
            field.insert((x, y));

            min_x = min_x.min(x);
            max_x = max_x.max(x);

            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
    }

    let mut interior: Option<(isize, isize)> = None;
    // calculate internal location
    'Outer: for y in min_y..=max_y {
        let mut last = false;
        for x in min_x..=max_x {
            let this = field.contains(&(x, y));
            if this && last { // don't want to handle double-row things
                continue 'Outer;
            } else if last { // we *were* one the border, now inside
                interior = Some((x, y));
                break 'Outer;
            }
            last = this;
        }
    }

    if let Some((ix, iy)) = interior {
        println!("({}, {}) is an interior point", ix, iy);
        // floodfill outwards
        let mut frontier: HashSet<(isize, isize)> = HashSet::new();
        frontier.insert((ix, iy));

        while frontier.len() > 0 {
            // Steps:
            // 1. Mark current as 'inside'
            // 2. Add adjacent points not already in the set as members
            let (x, y) = *frontier.iter().last().unwrap();
            frontier.remove(&(x, y));
            field.insert((x, y));
            for ox in -1..=1 {
                for oy in -1..=1 {
                    if ox == 0 && oy == 0 {
                        continue;
                    }

                    let adjacent = (ox+x, oy+y);
                    if !field.contains(&adjacent) && !frontier.contains(&adjacent) {
                        frontier.insert(adjacent);
                    }
                }
            }
        }

        println!("Size: {}", field.len());
    } else {
        println!("Couldn't find interior point");
    }

    /*
    // print out field
    let printable = field.iter().map(|line| String::from_iter(line));
    for line in printable {
        println!("{}", line);
    }*/
}

fn example() -> String {
    return "
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
".to_owned();
}

#[char_enum]
enum Moves {
    Up = 'U',
    Down = 'D',
    Left = 'L',
    Right = 'R'
}
impl Moves {
    fn offset(&self, x: isize, y: isize) -> (isize, isize) {
        match self {
            Moves::Up => (x, y-1),
            Moves::Down => (x, y+1),
            Moves::Left => (x-1, y),
            Moves::Right => (x+1, y)
        }
    }
}
