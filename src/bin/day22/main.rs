use std::{fs, cmp::Ordering};

fn main() {
    println!("AOC 2023 Day 22");

    let contents = fs::read_to_string("src/bin/day22/input.txt").expect("Failed to read input");

    let real = false;
    let mut pile = if real {BrickPile::load(&contents)} else {test_input()};
    pile.sort();
    //let mut pile2 = pile.clone();
    //pile2.sort();

    pile.print();
    println!("");

    pile.fall_down();
    println!("");

    pile.print();

    let mut safe_count = 0;
    let mut remaining: Vec<Brick> = vec![];
    println!("\n\nBrick status:");
    for brick in &pile.bricks {
        if brick.uniquely_supports.len() == 0 {
            println!("{} can be disintegrated", brick.id);
            safe_count += 1;
        } else {
            println!("{} can not be ...", brick.id);
            remaining.push(brick.clone());
        }
    }

    println!("\n\n");

    for b in remaining {
        b.print();
    }

    println!("\n\n");

    println!("Part 1: {}", safe_count);

    /*println!("\nPile 2:");
    pile2.fall_down();
    println!("");
    pile2.print();*/
}

/// (x0-x1) inclusive, etc
#[derive(Clone)]
struct Brick {
    x0: usize,
    x1: usize,

    y0: usize,
    y1: usize,

    z0: usize,
    z1: usize,

    id: usize,

    supported_by: Vec<usize>,
    uniquely_supports: Vec<usize>
}
impl Brick {
    fn parse(id: usize, line: &str) -> Brick {
        let (from, to) = line.split_once("~").unwrap();
        let mut x0_y0_z0 = from.split(",");
        let mut x1_y1_z1 = to.split(",");

        let x0 = x0_y0_z0.next().unwrap().parse().unwrap();
        let x1 = x1_y1_z1.next().unwrap().parse().unwrap();
        
        let y0 = x0_y0_z0.next().unwrap().parse().unwrap();
        let y1 = x1_y1_z1.next().unwrap().parse().unwrap();

        let z0 = x0_y0_z0.next().unwrap().parse().unwrap();
        let z1 = x1_y1_z1.next().unwrap().parse().unwrap();

        assert!(x0 <= x1);
        assert!(y0 <= y1);
        assert!(z0 <= z1);

        return Brick { x0, x1, y0, y1, z0, z1, id, supported_by: vec![], uniquely_supports: vec![] };
    }

    #[allow(dead_code)]
    fn volume(&self) -> usize {
        return (self.x1 - self.x0 + 1) * (self.y1 - self.y0 + 1) * (self.z1 - self.z0 + 1);
    }

    fn fall_down(&mut self, dist: usize) {
        self.z0 -= dist;
        self.z1 -= dist;
        println!("Brick {} falling down {} blocks", self.id, dist);
    }

    fn horizontal_overlap(&self, x0: usize, x1: usize, y0: usize, y1: usize) -> bool {
        if x1 < self.x0 || x0 > self.x1 || y1 < self.y0 || y0 > self.y1 {
            return false;
        }
        return true;
    }

    fn print(&self) {
        println!("{}:{},{},{}~{},{},{}", self.id, self.x0, self.y0, self.z0, self.x1, self.y1, self.z1);
    }
}

fn test_input() -> BrickPile {
    // slightly modified example to ensure that index != id, should return 6
    // ^ actually using built-in example for part 2
    return BrickPile::load("1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
"/*
2,8,48~2,8,49
1,8,2~3,8,2
"*/);
}

#[derive(Clone)]
struct BrickPile {
    bricks: Vec<Brick>,
    id_to_idx: Vec<usize>
}
impl BrickPile {
    fn load(data: &str) -> BrickPile {
        let bricks: Vec<Brick> = data.trim().split("\n").enumerate().map(|(i, l)| Brick::parse(i, l)).collect();

        let mut id_to_idx: Vec<usize> = vec![];

        for id in 0..bricks.len() {
            id_to_idx.push(id);
        }

        return BrickPile { bricks, id_to_idx };
    }

    #[allow(dead_code)]
    fn sort(&mut self) {
        self.bricks.sort_by(|a, b| {
            let initial = a.z0.partial_cmp(&b.z0).unwrap();
            if let Ordering::Equal = initial {
                return a.z1.partial_cmp(&b.z1).unwrap();
            }
            return initial;
        });

        for (i, brick) in self.bricks.iter().enumerate() {
            self.id_to_idx[brick.id] = i;
        }
    }

    #[allow(dead_code)]
    fn get_brick(&self, id: usize) -> Option<&Brick> {
        return Some(&self.bricks[self.id_to_idx[id]]);
        /*for brick in &self.bricks {
            if brick.id == id {
                return Some(brick);
            }
        }
        return None;*/
    }

    fn get_brick_mut(&mut self, id: usize) -> Option<&mut Brick> {
        return Some(&mut self.bricks[self.id_to_idx[id]]);
        /*for brick in &mut self.bricks {
            if brick.id == id {
                return Some(brick);
            }
        }
        return None;*/
    }

    /// WARN: MUST call self.sort() first
    fn fall_down(&mut self) {
        //self.sort();
        for i in 0..self.bricks.len() {
            let brick = &self.bricks[i];
            // Strategy:
            // find the highest z1 in the footprint < brick.zo, and fall down to rest on that
            let x0 = brick.x0;
            let x1 = brick.x1;
            let y0 = brick.y0;
            let y1 = brick.y1;
            let z0 = brick.z0;

            let mut supporters: Vec<usize> = vec![];

            let mut highest = 0;

            for brick in &self.bricks {
                if brick.z0 >= z0 {
                    break;
                }
                if brick.z1 >= z0 {
                    continue;
                }
                if brick.horizontal_overlap(x0, x1, y0, y1) {
                    if brick.z1 > highest {
                        supporters = vec![brick.id];
                        highest = brick.z1;
                    } else if brick.z1 == highest {
                        supporters.push(brick.id);
                    }
                }
            }

            self.bricks[i].supported_by = supporters;
            println!("\nbrick {} supported by: {:?}", self.bricks[i].id, self.bricks[i].supported_by);

            if self.bricks[i].supported_by.len() == 1 {
                let supporter_id = self.bricks[i].supported_by[0];
                self.get_brick_mut(supporter_id).unwrap().uniquely_supports.push(i);
            }

            self.bricks[i].fall_down(z0 - (highest + 1));
        }
    }

    fn print(&self) {
        for brick in &self.bricks {
            brick.print();
        }
    }
}
