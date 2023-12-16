use std::{fs, fmt::{Display, Error, Formatter}};

fn main() {
    println!("AOC 2023 Day 12");

    let contents = fs::read_to_string("src/bin/day12/input.txt").expect("Failed to read input");
    debugging();

    let records: Vec<Record> = contents.trim().split("\n").map(|s| Record::parse(s)).collect();
    let sum: usize = records.iter().map(|r| r.arrangements()).sum();
    println!("Total arrangement count: {}", sum);

    let sum2: usize = records.iter().map(|r| r.expand().arrangements()).sum();
    println!("Total expanded arrangement count: {}", sum2);
}

fn debugging() {
    let r = Record::parse("?###???????? 3,2,1");
    let descriptions = r.variant_records().filter(|r| r.is_valid());
    println!("{}", r);
    for d in descriptions {
        println!("{}", d);
    }
}

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
        for _ in 0..5 {
            new_conditions.extend(self.copy_conditions());
            new_cont_damage.extend(self.copy_damage());
        }
        return Record { conditions: new_conditions, contiguous_damage: new_cont_damage };
    }

    fn arrangements(&self) -> usize {
        return self.variant_records().filter(|r| r.is_valid()).count();
    }

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

    fn variants(&self) -> Box<dyn Iterator<Item = Vec<Symbol>> + '_> {
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
    }

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
    assert_eq!(1, Record::parse("???.### 1,1,3").arrangements());
    assert_eq!(4, Record::parse(".??..??...?##. 1,1,3").arrangements());
    assert_eq!(1, Record::parse("?#?#?#?#?#?#?#? 1,3,1,6").arrangements());
    assert_eq!(4, Record::parse("????.######..#####. 1,6,5").arrangements());
    assert_eq!(10, Record::parse("?###???????? 3,2,1").arrangements());
    assert_eq!(506250, Record::parse("?###???????? 3,2,1").expand().arrangements());
}
