use std::fs;

fn main() {
    println!("AOC 2023 Day 3");
    let schem: &mut Schematic = &mut load_schematic("src/bin/day3/input.txt");
    println!("Part 1 sum: {}", schem.process());
    println!("Part 2 sum: {}", schem.count_gears());
}

#[derive(Debug)]
struct NumberStart {
    value: u32,
    is_part: bool
}

#[derive(Copy, Clone, Debug)]
struct Coord {
    row: usize,
    column: usize
}

#[derive(Debug)]
struct GearData {
    num_count: u32,
    product: u32
}

impl GearData {
    fn mesh_with(&mut self, number: u32) {
        if self.num_count == 0 {
            self.product = number;
        } else if self.num_count == 1 {
            self.product *= number;
        } else {
            self.product = 0;
        }
        self.num_count += 1;
    }
}

#[derive(Debug)]
enum Parts {
    Blank,
    Symbol(char),
    Gear(GearData),
    NumberStart(NumberStart),
    NumberContinue(Coord)
}

struct Schematic {
    width: usize,
    height: usize,
    data: Vec<Vec<Parts>>,
    symbols: Vec<Coord>,
    result: Option<u32>,
}

fn load_schematic(fname: &str) -> Schematic {
    let contents = fs::read_to_string(fname).expect("Failed to load file");
    return Schematic::create(&contents);
}

impl Schematic {
    fn create(data: &str) -> Schematic {
        let lines: Vec<&str> = data.split("\n").filter(|x| !x.is_empty()).collect();
        let height = lines.len();
        let width = lines[0].chars().count();
        let mut data: Vec<Vec<Parts>> = vec![];
        let mut symbols: Vec<Coord> = vec![];
        for row in 0..height {
            let mut row_data: Vec<Parts> = vec![];
            let mut line = lines[row].chars();
            let mut building_number: Option<Coord> = None;
            for column in 0..width {
                let chr = line.next().unwrap();
                let digit = chr.to_digit(10);
                if chr == '.' {
                    row_data.push(Parts::Blank);
                    building_number = None;
                } else if chr == '*' {
                    row_data.push(Parts::Gear(GearData { num_count: 0, product: 0 }));
                    symbols.push(Coord { row, column });
                    building_number = None;
                } else if let Some(d) = digit {
                    match building_number {
                        Some(coord) => {
                            if let Parts::NumberStart(num_data) = &mut row_data[coord.column] {
                                num_data.value *= 10;
                                num_data.value += d;
                            } else {
                                panic!("Incorrect continuation coord");
                            }
                            row_data.push(Parts::NumberContinue(coord));
                        },
                        None => {
                            row_data.push(Parts::NumberStart(NumberStart { value: d, is_part: false }));
                            building_number = Some(Coord { row, column });
                        },
                    };
                } else {
                    row_data.push(Parts::Symbol(chr));
                    symbols.push(Coord { row, column });
                    building_number = None;
                }
            }
            data.push(row_data);
        }
        return Schematic {width, height, data, symbols, result: None };
    }

    fn process(&mut self) -> u32 {
        if let Some(c) = self.result {
            return c;
        }
        let mut count: u32 = 0;
        for coord in &self.symbols {
            for row_offset in -1..=1 {
                for column_offset in -1..=1 {
                    if row_offset == 0 && column_offset == 0 {
                        continue;
                    }
                    let row: i32 = (coord.row as i32) + row_offset;
                    let column: i32 = (coord.column as i32) + column_offset;
                    if row < 0 || column < 0 || row as usize >= self.height || column as usize >= self.width {
                        continue;
                    }
                    let mut coord_to_set: Option<Coord> = None;
                    let mut count_set: Option<u32> = None;
                    match &mut self.data[row as usize][column as usize] {
                        Parts::NumberStart(num_data) => {
                            if !num_data.is_part {
                                num_data.is_part = true;
                                count += num_data.value;
                                count_set = Some(num_data.value);
                            }
                        },
                        Parts::NumberContinue(c) => {
                            coord_to_set = Some(*c);
                        },
                        _ => {},
                    };
                    if let Some(c) = coord_to_set {
                        if let Parts::NumberStart(num_data) = &mut self.data[c.row][c.column] {
                            if !num_data.is_part {
                                num_data.is_part = true;
                                count += num_data.value;
                                count_set = Some(num_data.value);
                            }
                        }
                    }
                    if let Some(cs) = count_set {
                        if let Parts::Gear(gear_data) = &mut self.data[coord.row][coord.column] {
                            gear_data.mesh_with(cs);
                        }
                    }
                }
            }
        }
        self.result = Some(count);
        return count;
    }

    fn count_gears(&self) -> u32 {
        let mut gear_sum: u32 = 0;
        for row in 0..self.height {
            for column in 0..self.width {
                if let Parts::Gear(gear_data) = &self.data[row][column] {
                    if gear_data.num_count == 2 {
                        gear_sum += gear_data.product;
                    }
                }
            }
        }
        return gear_sum;
    }

    #[allow(dead_code)]
    fn debug_print(&self) {
        for row in 0..self.height {
            print!("[{}]: ", row);
            for column in 0..self.width {
                let part = &self.data[row][column];
                print!("{:#?}, ", part);
            }
            println!("");
        }
    }
}

#[test]
fn process_works() {
    let s: &mut Schematic = &mut load_schematic("src/bin/day3/test.txt");
    s.process();
    s.debug_print();
    assert_eq!(4361, s.process());
    assert_eq!(467835, s.count_gears());
}
