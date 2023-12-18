use std::{fs, ops::Add};
use char_enum_impl::data_enum;
use utils::{DijkstraData, DijkstraNode};

fn main() {
    println!("AOC 2023 Day 17");

    let contents = fs::read_to_string("src/bin/day17/input.txt").expect("Failed to read input");
    let map = Map::parse(&contents);

    let best = map.best_distance();
    println!("Part 1: {}", best);
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[data_enum((isize, isize))]
enum Direction {
    North = (0, -1),
    South = (0, 1),
    East = (1, 0),
    West = (-1, 0)
}
impl Direction {
    fn right_turn(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North
        }
    }

    fn left_turn(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North
        }
    }

    #[inline(always)]
    fn values() -> Vec<Direction> {
        return vec![Direction::North, Direction::East, Direction::South, Direction::West];
    }
}
impl Add<(u8, u8)> for Direction {
    type Output = (i16, i16);
    
    fn add(self, other: (u8, u8)) -> <Self as Add<(u8, u8)>>::Output {
        let (row, column) = other;
        let (ocol, orow) = self.value();
        return ((row as isize + orow) as i16, (column as isize + ocol) as i16);
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Node {
    row: u8,
    column: u8,
    travelling_direction: Direction,
    straight_dist: u8,
}
impl Node {
    fn maybe_adjacent(&self, map: &Map, direction: Direction) -> Option<(Self, usize)> {
        if self.travelling_direction == direction { // forward

            if self.straight_dist >= 3 {
                return None;
            }

            let (nr, nc) = direction + (self.row, self.column);
            if nr < 0 || nc < 0 || nr >= map.height as i16 || nc >= map.width as i16 { // bounds check
                return None;
            }
            let (nr, nc) = (nr as u8, nc as u8);

            return Some((
                    Node {
                        row: nr,
                        column: nc,
                        travelling_direction: direction,
                        straight_dist: self.straight_dist + 1
                    }, map.heat_loss[nr as usize][nc as usize] as usize));
        } else if self.travelling_direction.right_turn() == direction
            || self.travelling_direction.left_turn() == direction { // right or left

            let (nr, nc) = direction + (self.row, self.column);
            if nr < 0 || nc < 0 || nr >= map.height as i16 || nc >= map.width as i16 { // bounds check
                return None;
            }
            let (nr, nc) = (nr as u8, nc as u8);

            return Some((
                    Node {
                        row: nr,
                        column: nc,
                        travelling_direction: direction,
                        straight_dist: 1
                    }, map.heat_loss[nr as usize][nc as usize] as usize));
        } else { // cannot go backwards
            return None;
        }
    }
}
impl DijkstraNode<Map> for Node {
    fn get_connected(&self, context: &Map)-> Vec<(Self, usize)> {
        let mut out = vec![];
        for direction in Direction::values() {
            if let Some(v) = self.maybe_adjacent(context, direction) {
                out.push(v);
            }
        }
        return out;
    }
}

struct Map {
    /// indexed [row][column]
    heat_loss: Vec<Vec<u8>>,
    width: u8,
    height: u8
}
impl Map {
    fn parse(data: &str) -> Map {
        let lines: Vec<&str> = data.trim().split("\n").collect();
        let width = lines[0].len() as u8;
        let height = lines.len() as u8;

        let heat_loss: Vec<Vec<u8>> = lines.iter()
            .map(|l| l.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
            )
            .collect();
        return Map { heat_loss, width, height };
    }

    fn best_distance(self) -> usize {
        let target_row = self.height - 1;
        let target_column = self.width - 1;
        let initial = Node { row: 0, column: 0, travelling_direction: Direction::East, straight_dist: 0};
        let d = DijkstraData::dijkstra(initial, self);
        let best = *d.best_distance.iter()
            .filter(|(n, _)| n.row == target_row && n.column == target_column)
            .map(|(_, d)| d)
            .min().expect("Pathfinding failed");
        return best;
    }
}

#[test]
fn correct_distance() {
    let map = Map::parse("
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
");
    assert_eq!(102, map.best_distance());
}
