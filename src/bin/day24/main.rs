use std::{fs, fmt::{Display, Formatter}};

fn main() {
    println!("AOC 2023 Day 24");

    let real = true;
    let contents: String;
    let bounds: Bounds;
    if real {
        contents = fs::read_to_string("src/bin/day24/input.txt").expect("Failed to read input");
        bounds = Bounds::square(200000000000000.0, 400000000000000.0);
    } else {
        contents = fs::read_to_string("src/bin/day24/example.txt").expect("Failed to read input");
        bounds = Bounds::square(7.0, 27.0);
    }

    let entries: Vec<Entry> = contents.trim().split("\n").map(|l| Entry::parse(l)).collect();
    for entry in &entries {
        println!("{}", entry);
    }

    println!("\n\n");

    let mut sum_ok: usize = 0;

    for i in 0..entries.len() {
        for j in i+1..entries.len() {
            let a = &entries[i];
            let b = &entries[j];
            println!("Hailstone A: {}", a);
            println!("Hailstone B: {}", b);
            let result = compute_crossing_state(a, b, &bounds);
            println!("{}\n", result);
            if let CrossingState::FutureInside(_, _) = result {
                sum_ok += 1;
            }
        }
    }
    println!("{} hailstones' future paths cross inside the boundaries (part 1)", sum_ok);
}

trait ApproxEq<T> {
    fn approx_eq(self, other: T) -> bool;
}

impl ApproxEq<f64> for f64 {
    fn approx_eq(self, other: f64) -> bool {
        return (self - other).abs() < 0.0001;
    }
}

enum CrossingState {
    Parallel,
    PastA,
    PastB,
    PastBoth,
    FutureInside(f64, f64),
    FutureOutside(f64, f64)
}
impl Display for CrossingState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Hailstones' paths ")?;
        return match self {
            Self::Parallel => f.write_str("are parallel; they never intersect."),
            Self::PastA => f.write_str("crossed in the past for hailstone A."),
            Self::PastB => f.write_str("crossed in the past for hailstone B."),
            Self::PastBoth => f.write_str("crossed in the past for both hailstones."),
            Self::FutureInside(x, y) => f.write_fmt(format_args!("will cross inside the test area (at x={}, y={}).", x, y)),
            Self::FutureOutside(x, y) => f.write_fmt(format_args!("will cross outside the test area (at x={}, y={}).", x, y)),
        };
    }
}

struct Entry {
    p_x: f64,
    p_y: f64,
    p_z: f64,

    v_x: f64,
    v_y: f64,
    v_z: f64
}
impl Entry {
    fn parse(data: &str) -> Entry {
        let (p, v) = data.split_once("@").unwrap();

        let mut p = p.trim().split(",").map(|x| x.trim().parse::<i64>().unwrap() as f64);
        let mut v = v.trim().split(",").map(|x| x.trim().parse::<i64>().unwrap() as f64);

        let p_x = p.next().unwrap();
        let p_y = p.next().unwrap();
        let p_z = p.next().unwrap();

        let v_x = v.next().unwrap();
        let v_y = v.next().unwrap();
        let v_z = v.next().unwrap();

        return Entry { p_x, p_y, p_z, v_x, v_y, v_z };
    }

    fn is_future(&self, x: f64) -> bool {
        if self.v_x > 0.0 {
            return x > self.p_x;
        } else {
            return x < self.p_x;
        }
    }

    #[inline(always)]
    fn is_past(&self, x: f64) -> bool {
        !self.is_future(x)
    }
}
impl Display for Entry {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}, {} @ {}, {}, {}", self.p_x, self.p_y, self.p_z, self.v_x, self.v_y, self.v_z)
    }
}

struct Bounds {
    x0: f64,
    y0: f64,

    x1: f64,
    y1: f64
}
impl Bounds {
    fn square(min: f64, max: f64) -> Bounds {
        return Bounds { x0: min, y0: min, x1: max, y1: max };
    }

    fn contains(&self, x: f64, y: f64) -> bool {
        return self.x0 <= x && x <= self.x1 && self.y0 <= y && y <= self.y1;
    }
}

fn crossing_coord(a: &Entry, b: &Entry) -> (f64, f64) {
    let m1 = a.v_y / a.v_x;
    let m2 = b.v_y / b.v_x;

    let x = {
        let over = (a.p_x * m1) - a.p_y - (b.p_x * m2) + b.p_y;
        let under = m1 - m2;
        over / under
    };
    let y = (m1 * x) - (m1 * a.p_x) + a.p_y;

    return (x, y);
}

fn compute_crossing_state(a: &Entry, b: &Entry, bounds: &Bounds) -> CrossingState {
    if (a.v_y * b.v_x).approx_eq(b.v_y * a.v_x) {
        return CrossingState::Parallel;
    }
    let (x, y) = crossing_coord(a, b);

    if a.is_past(x) {
        if b.is_past(x) {
            return CrossingState::PastBoth;
        } else {
            return CrossingState::PastA;
        }
    } else if b.is_past(x) {
        return CrossingState::PastB;
    } else { // future
        if bounds.contains(x, y) {
            return CrossingState::FutureInside(x, y);
        } else {
            return CrossingState::FutureOutside(x, y);
        }
    }
}
