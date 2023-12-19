use std::fs;

use char_enum_impl::char_enum;

fn main() {
    println!("AOC 2023 Day 18");

    let contents: String;
    if true {
        contents = fs::read_to_string("src/bin/day18/input.txt").expect("Failed to read input");
    } else {
        contents = example();
    }
    //Vec<(Moves, u8)> 
    let data: Vec<(Moves, usize)> = contents.trim().split("\n")
        .map(|line| line
            .strip_suffix(")").unwrap()
            .split_once(" (#").unwrap()
        ).map(|(_, color)| color.to_owned())
        .map(|color| (color[0..5].to_owned(), color.chars().nth(5).unwrap()))
        .map(|(enc_dist, dir)| (Moves::decode(dir), usize::from_str_radix(&enc_dist, 16).unwrap()))
        .collect();

    /*
     * Algorithm, thanks to the lovely people on r/adventofcode
     * (No way do I know about Pick's theorem and the shoelace formula)
     *
     * specifically helpful posts/comments:
     * u/Boojum: https://www.reddit.com/r/adventofcode/comments/18l2tap/comment/kdv5bzi/
     * u/SEGV_AGAIN: https://www.reddit.com/r/adventofcode/comments/18l8mao/2023_day_18_intuition_for_why_spoiler_alone/
     *
     * total_area = inner_area + (perimeter/2) + 1
     *
     * this works b/c Pick's theorem calculates the area of a shape with edges *centered* in the
     * perimeter trenches
     *
     * this results in perimeter trenches having the following extra 'outside' area
     * 1/2 in edge pieces, as such:
     * X|#
     *
     * X|#
     *
     * 3/4 in convex corners, as such:
     * X X
     *  +-
     * X|#
     *
     * 1/4 in concave corners, as such
     * X|#
     * -+
     * # #
     *
     * convave and convex corners balance out, resulting in an average 1/2 contribution from each
     * and there are 4 convex corners (see a rectangle), so that adds another 1 (2 units of area
     * are already included in the perimeter/2 calculation)
     */
    
    /*
     * Shoelace (triangle variant)
     * for every edge from (x1, y1) to (x2, y2)
     * add (x1*y2 - x2*y1)/2 to the total
     */

    let mut sum: isize = 0; // due to some fun little shoelace-shenannigans, this could (probably)
                            // become negative at some point in time
                            // NOTE: to avoid floating point misery, divide TOTAL_AREA in half at the very end
                            // formula becomes: (sum + perimeter + 2) / 2
    let mut perimeter: usize = 0;

    // calculate all the edges
    let (mut x1, mut y1) = (0, 0);
    for (dir, dist) in data {
        perimeter += dist;

        let (x2, y2) = dir.offset(x1, y1, dist as isize);
        sum += x1*y2 - x2*y1;

        // prepare for next iter
        x1 = x2;
        y1 = y2;
    }

    let sum = sum as usize;
    let total = (sum + perimeter + 2) / 2;
    println!("Total area: {}", total);
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
    Up = '3',
    Down = '1',
    Left = '2',
    Right = '0'
}
impl Moves {
    fn offset(&self, x: isize, y: isize, dist: isize) -> (isize, isize) {
        match self {
            Moves::Up => (x, y-dist),
            Moves::Down => (x, y+dist),
            Moves::Left => (x-dist, y),
            Moves::Right => (x+dist, y)
        }
    }
}
