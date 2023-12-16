use std::{fs, fmt::{Display, Error, Formatter}};

use memoize::memoize;

fn main() {
    println!("AOC 2023 Day 12");

    let contents = fs::read_to_string("src/bin/day12/input.txt").expect("Failed to read input");
    //debugging();

    let records: Vec<Record> = contents.trim().split("\n").map(|s| Record::parse(s)).collect();
    let sum: u64 = records.iter().map(|r| r.arrangements()).sum();
    println!("Total arrangement count: {}", sum);

    let sum2: u64 = records.iter().map(|r| r.expand().arrangements()).sum();
    println!("Total expanded arrangement count: {}", sum2);
}

/*fn debugging() {
    let r = Record::parse("?###???????? 3,2,1");
    let descriptions = r.variant_records().filter(|r| r.is_valid());
    println!("{}", r);
    for d in descriptions {
        println!("{}", d);
    }
}*/

#[derive(PartialEq, Clone, Copy)]
enum Symbol {
    Operational,
    Damaged,
    Unknown
}
impl Symbol {
    fn decode(chr: char) -> Symbol {
        return match chr {
            '.' => Symbol::Operational,
            '#' => Symbol::Damaged,
            '?' => Symbol::Unknown,
            _ => panic!("Unknown symbol")
        }
    }

    fn encode(&self) -> char {
        return match self {
            Symbol::Operational => '.',
            Symbol::Damaged => '#',
            Symbol::Unknown => '?'
        };
    }
}

#[allow(dead_code)]
fn colorize(input: &str, r: u8, g: u8, b: u8) -> String {
    return "\x1b[38;2;".to_owned()+&r.to_string()+";"+&g.to_string()+";"+&b.to_string()+"m"+input+"\x1b[0m";
}

struct Record {
    conditions: Vec<Symbol>,
    contiguous_damage: Vec<u64>
}
impl Record {
    fn parse(line: &str) -> Record {
        let (conditions_dat, damage_dat) = line.trim().split_once(" ").unwrap();
        let conditions: Vec<Symbol> = conditions_dat.chars().map(|c| Symbol::decode(c)).collect();
        let contiguous_damage: Vec<u64> = damage_dat.split(",").map(|s| s.parse::<u64>().unwrap()).collect();
        return Record { conditions, contiguous_damage };
    }

    fn expand(&self) -> Record {
        let mut new_conditions: Vec<Symbol> = vec![];
        let mut new_cont_damage: Vec<u64> = vec![];
        for i in 1..=5u8 {
            new_conditions.extend(self.copy_conditions());
            if i < 5 {
                new_conditions.push(Symbol::Unknown);
            }
            new_cont_damage.extend(self.copy_damage());
        }
        return Record { conditions: new_conditions, contiguous_damage: new_cont_damage };
    }

    /*fn arrangements(&self) -> usize {
        return self.variant_records().filter(|r| r.is_valid()).count();
    }*/

    #[inline]
    fn copy_conditions(&self) -> Vec<Symbol> {
        let mut new: Vec<Symbol> = vec![];
        new.reserve_exact(self.conditions.len());
        for s in &self.conditions {
            new.push(*s);
        }
        return new;
    }

    #[inline]
    fn copy_damage(&self) -> Vec<u64> {
        let mut new: Vec<u64> = vec![];
        new.reserve_exact(self.contiguous_damage.len());
        for s in &self.contiguous_damage {
            new.push(*s);
        }
        return new;
    }

    #[inline]
    fn clone(&self) -> Record {
        return Record { conditions: self.copy_conditions(), contiguous_damage: self.copy_damage() };
    }

    fn arrangements(&self) -> u64 {
        #[cfg(test)]
        println!("\n\n\n\n");
        return self.clone().mut_ok_subvariants(0);
    }

    fn mut_ok_subvariants(&mut self, depth: usize) -> u64 {
        #[cfg(test)]
        let colors = [
            (255, 0, 0),
            (255, 127, 0),
            (255, 255, 0),
            (0, 255, 0),
            (0, 0, 255),
            (75, 0, 130),
            (148, 0, 211)
        ];
        #[cfg(test)]
        let mut indentation = "".to_owned();
        #[cfg(test)]
        for d in 0..depth {
            let (r, g, b) = colors[d%colors.len()];
            indentation += &colorize(" |", r, g, b);
        }
        #[cfg(test)]
        println!("{}Checking {}", indentation, self);
        while self.conditions.len() > 0 && self.conditions[0] == Symbol::Operational { // strip leading ....
            self.conditions.remove(0);
        }
        if self.conditions.len() == 0 {
            if self.contiguous_damage.len() == 0 {
                #[cfg(test)]
                println!("{}> {}", indentation, colorize("ran out simultaneously", 0, 255, 0));
                return 1;
            } else {
                #[cfg(test)]
                println!("{}> ran out of damaged area", indentation);
                return 0;
            }
        }
        if self.contiguous_damage.len() == 0 {
            for s in &self.conditions {
                if *s == Symbol::Damaged {
                    return 0;
                }
            }
            #[cfg(test)]
            println!("{}> {}", indentation, colorize("ran out with '.' padding", 0, 255, 0));
            return 1;
        }
        match self.conditions.remove(0) {
            Symbol::Operational => panic!("Operational should be stripped"),
            Symbol::Damaged => {
                let damage_len = self.contiguous_damage[0] as usize;
                if self.conditions.len() < damage_len-1 {
                    #[cfg(test)]
                    println!("{}> not enough conditions", indentation);
                    return 0;
                }
                // in this case conditions[0..damage_len-1] have to all be
                // '#' or '?' (coerced to '#'); AKA none of them can be Operational
                // the -1 is because we pre-remove the leading '#'
                for idx in 0..damage_len-1 {
                    if self.conditions[idx] == Symbol::Operational {
                        #[cfg(test)]
                        println!("{}> found operational at idx {} in damaged area of length {}, self: {}", indentation, idx, damage_len, self);
                        return 0;
                    }
                }
                // then, conditions[damage_len] has to either a) not exist or b) equal '.' or '?'
                // and then check children
                if self.conditions.len() == damage_len-1 {
                    if self.contiguous_damage.len() == 1 {
                        #[cfg(test)]
                        println!("{}> {}", indentation, colorize("perfect end length", 0, 255, 0));
                        return 1;
                    } else {
                        #[cfg(test)]
                        println!("{}> not enough space for conditions", indentation);
                        return 0;
                    }
                }
                if self.conditions[damage_len-1] == Symbol::Damaged {
                    #[cfg(test)]
                    println!("{}> damaged at end of desired damage length", indentation);
                    return 0;
                }
                let mut child = Record {
                    conditions: self.conditions[damage_len..self.conditions.len()].to_vec(),
                    contiguous_damage: self.contiguous_damage[1..self.contiguous_damage.len()].to_vec()
                };
                return child.mut_ok_subvariants(depth+1);
            },
            Symbol::Unknown => {
                let mut rc0: Vec<Symbol> = vec![]; // operational variant, pre-strip
                let mut rc1: Vec<Symbol> = vec![Symbol::Damaged];

                rc0.extend(self.copy_conditions());
                rc1.extend(self.copy_conditions());

                #[cfg(test)]
                println!("{} {}", indentation, colorize(&format!("[ '.{}' ]", Record::variant_to_string(&rc0)), 255, 0, 255));
                let mut r0 = Record { conditions: rc0, contiguous_damage: self.copy_damage() };
                let c0 = r0.mut_ok_subvariants(depth+1);
                #[cfg(test)]
                println!("{} {}", indentation, colorize(&format!("[ '{}' ]", Record::variant_to_string(&rc1)), 255, 0, 255));
                let mut r1 = Record { conditions: rc1, contiguous_damage: self.copy_damage() };
                let c1 = r1.mut_ok_subvariants(depth+1);
                return c0 + c1;
            },
        };
    }

    /*fn variants(&self) -> Box<dyn Iterator<Item = Vec<Symbol>> + '_> {
        let unknown_count = self.conditions.iter().filter(|s| **s == Symbol::Unknown).count();
        if unknown_count == 0 {
            return Box::new(vec![self.copy_conditions()].into_iter());
        }
        let a = ( 0_usize..(1<<unknown_count) ).map(|v| {
            let mut new_conditions: Vec<Symbol> = vec![];
            new_conditions.reserve_exact(self.conditions.len());
            let mut counter: usize = 0;
            for s in &self.conditions {
                if *s == Symbol::Unknown {
                    let mask: usize = 1<<counter;
                    counter += 1;
                    if v & mask != 0 {
                        new_conditions.push(Symbol::Operational);
                    } else {
                        new_conditions.push(Symbol::Damaged);
                    }
                } else {
                    new_conditions.push(*s);
                }
            }
            return new_conditions;
        });
        return Box::new(a);
    }

    fn is_valid(&self) -> bool {
        let mut cont: u64 = 0;
        let mut idx: usize = 0;

        let mut c: Vec<Symbol> = vec![];
        c.extend(self.copy_conditions());
        c.push(Symbol::Operational);

        for s in c {
            match s {
                Symbol::Operational => {
                    if cont != 0 {
                        if idx >= self.contiguous_damage.len() {
                            return false;
                        }
                        if cont != self.contiguous_damage[idx] {
                            return false;
                        }
                        idx += 1;
                        cont = 0;
                    }
                },
                Symbol::Damaged => {
                    cont += 1;
                },
                Symbol::Unknown => panic!("Can't check validity of unknown parts")
            };
        }

        return idx == self.contiguous_damage.len();
    }

    fn variant_records(&self) -> Box<dyn Iterator<Item = Record> + '_> {
        return Box::new(self.variants().map(|v| Record { conditions: v, contiguous_damage: self.copy_damage() }));
    }*/

    fn variant_to_string(data: &Vec<Symbol>) -> String {
        return String::from_iter(data.iter().map(|s| s.encode()));
    }
}
impl Display for Record {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> Result<(), Error> {
        let damage_desc = self.contiguous_damage.iter().map(|v| format!("{}", v)).collect::<Vec<String>>().join(",");
        return match write!(formatter, "{} {}", Record::variant_to_string(&self.conditions), damage_desc) {
            Ok(_) => Ok(()),
            Err(_) => Err(Error)
        }
    }
}

#[test]
fn arrangement_count() {
    assert_eq!(1, Record::parse(".##.?#??.#.?# 2,1,1,1").arrangements());
    assert_eq!(1, Record::parse("#??.### 1,1,3").arrangements());
    assert_eq!(1, Record::parse("???.### 1,1,3").arrangements());
    assert_eq!(4, Record::parse(".??..??...?##. 1,1,3").arrangements());
    assert_eq!(1, Record::parse("?#?#?#?#?#?#?#? 1,3,1,6").arrangements());
    assert_eq!(4, Record::parse("????.######..#####. 1,6,5").arrangements());
    assert_eq!(10, Record::parse("###???????? 3,2,1").arrangements());
    assert_eq!(10, Record::parse("?###???????? 3,2,1").arrangements());
}

#[test]
fn argument_count_expanded() {
    assert_eq!(16, Record::parse("????.#...#... 4,1,1").expand().arrangements());
    assert_eq!(16384, Record::parse(".??..??...?##. 1,1,3").expand().arrangements());
    assert_eq!(1, Record::parse("?#?#?#?#?#?#?#? 1,3,1,6").expand().arrangements());
    assert_eq!(506250, Record::parse("?###???????? 3,2,1").expand().arrangements());
}
