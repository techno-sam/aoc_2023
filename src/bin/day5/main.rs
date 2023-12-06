use std::{collections::HashMap, fs};

fn main() {
    println!("AOC 2023 Day 5");
    let contents = fs::read_to_string("src/bin/day5/input.txt").expect("Failed to read input");
    let almanac: Almanac = Almanac::load(&contents);

    let mut min: Vec<(u64, u64)> = almanac.seeds.iter()
        .map(|seed| (*seed, almanac.map("seed", "location", *seed)))
        .collect();
    min.sort_by(|(_s1, l1), (_s2, l2)| l1.partial_cmp(l2).unwrap());
    let (_, mininum_loc) = min[0];
    println!("Part 1 minimum location: {}", mininum_loc);

    let seed_id = almanac.name_id.get("seed").unwrap();
    let location_id = almanac.name_id.get("location").unwrap();
    let mappers = almanac.mapper_chain(seed_id, location_id);
    println!("Created mappers");
    let ranges = almanac.expand_ranges();
    let minimum_loc2 = RangeMap::apply_sequence_to_ranges_iter(ranges, mappers)
        .map(|r| r.start)
        .min().unwrap();
    println!("Part 2 minimum location: {}", minimum_loc2);
}


struct RangeMapEntry {
    dest_start: i64,
    source_start: i64,
    range_length: i64,
}
impl RangeMapEntry {
    fn load(entry: &str) -> RangeMapEntry {
        let mut parts = entry.trim().split(" ").filter(|s| !s.is_empty());
        let dst = parts.next().unwrap().parse::<u64>().unwrap() as i64;
        let src = parts.next().unwrap().parse::<u64>().unwrap() as i64;
        let range_length = parts.next().unwrap().parse::<u64>().unwrap() as i64;
        return RangeMapEntry { dest_start: dst, source_start: src, range_length };
    }

    fn map(&self, val: u64) -> Option<u64> {
        let mut v = val as i64;
        if v < self.source_start || v >= self.source_start + self.range_length {
            return None;
        }
        v -= self.source_start;
        v += self.dest_start;
        return Some(v as u64);
    }

    /// inclusive
    #[inline]
    fn source_end(&self) -> i64 {
        return self.source_start + self.range_length - 1;
    }

    #[inline]
    fn offset(&self) -> i64 {
        return self.dest_start - self.source_start;
    }
}

#[allow(dead_code)]
struct RangeMap {
    name: String,
    entries: Vec<RangeMapEntry> // MUST be sorted by source_start
}
#[allow(dead_code)]
impl RangeMap {
    fn load(name: &str, entries: &str) -> RangeMap {
        let mut ent: Vec<RangeMapEntry> = entries.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| RangeMapEntry::load(s))
            .collect();
        ent.sort_by(|a, b| a.source_start.partial_cmp(&b.source_start).unwrap());
        return RangeMap { name: name.to_owned(), entries: ent };
    }

    fn map(&self, val: u64) -> u64 {
        for entry in &self.entries {
            if let Some(v) = entry.map(val) {
                return v;
            }
        }
        return val;
    }

    fn map_in_place(&self, val: &mut u64) {
        *val = self.map(*val);
    }

    fn map_ranges(&self, range: SeedRange) -> Vec<SeedRange> {
        let mut out: Vec<SeedRange> = vec![];
        // let orig_range = SeedRange::n(range.start, range.end, range.done_processing);
        let mut todo: Vec<SeedRange> = vec![range];

        for entry in &self.entries {
            let mut new_todo: Vec<SeedRange> = vec![];
            for element in todo {
                let split = element.split_over(entry.source_start, entry.source_end());
                for mut s in split {
                    if s.done_processing {
                        if entry.source_start <= s.start && s.end <= entry.source_end() {
                            s.offset(entry.offset());
                        }
                        out.push(s);
                    } else {
                        new_todo.push(s);
                    }
                }
            }
            todo = new_todo;
        }
        out.extend(todo);
        // reset done_processing flag for next
        //print!("{} Split range {:?} into: ", self.name, orig_range);
        for r in &mut out {
            r.done_processing = false;
            //print!("{:?}, ", r);
        }
        //println!("");
        return out;
    }


    fn apply_sequence(v: &mut u64, mappers: &Vec<&RangeMap>) {
        for mapper in mappers {
            mapper.map_in_place(v);
        }
    }

    fn apply_sequence_to_ranges(v: Vec<SeedRange>, mappers: &Vec<&RangeMap>) -> Vec<SeedRange> {
        let mut out: Vec<SeedRange> = v;
        for mapper in mappers {
            let mut next_out: Vec<SeedRange> = vec![];
            for range in out {
                next_out.extend(mapper.map_ranges(range));
            }
            out = next_out;
        }
        return out;
    }

    fn apply_sequence_to_ranges_iter<'a>(iter: impl Iterator<Item = SeedRange> + 'a, mappers: Vec<&'a RangeMap>)
        -> Box<dyn Iterator<Item = SeedRange> + 'a> {
        let mut it: Box<dyn Iterator<Item = SeedRange>> = Box::new(iter);
        for mapper in mappers {
            it = Box::new(it.map(|r| mapper.map_ranges(r)).flatten());
        }
        return it;
    }
}

#[allow(dead_code)]
fn get_test_output() -> String {
    return "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
".to_string();
}

struct Almanac {
    seeds: Vec<u64>,
    // further optimization: string comparison is slow, use ids for map names instead
    maps: HashMap<u64, (u64, RangeMap)>,
    name_id: HashMap<String, u64>
}
impl Almanac {
    fn load(data: &str) -> Almanac {
        let (seeds, maps_str) = data.trim().split_once("\n\n").unwrap();
        let maps = maps_str.split("\n\n");
        let mut next_id: u64 = 0;
        let mut all_maps: HashMap<u64, (u64, RangeMap)> = HashMap::new();
        let mut name_id: HashMap<String, u64> = HashMap::new();
        for map in maps {
            let (title, entries) = map.split_once(" map:\n").unwrap();
            let (from, to) = title.split_once("-to-").unwrap();
            let map = RangeMap::load(title, entries);
            let from_id: u64 = match name_id.get(from) {
                Some(&id) => id,
                None => {
                    let id = next_id;
                    next_id += 1;
                    name_id.insert(from.to_owned(), id);
                    id
                }
            };
            let to_id: u64 = match name_id.get(to) {
                Some(&id) => id,
                None => {
                    let id = next_id;
                    next_id += 1;
                    name_id.insert(to.to_owned(), id);
                    id
                }
            };
            all_maps.insert(from_id, (to_id, map));
        }
        let seed_vals: Vec<u64> = seeds.split(" ")
            .filter(|s| !s.is_empty())
            .filter(|s| s != &"seeds:")
            .map(|s| s.parse::<u64>().expect(&("Failed to parse [".to_owned()+s+"]")))
            .collect();
        return Almanac { seeds: seed_vals, maps: all_maps, name_id };
    }

    fn map(&self, from_name: &str, to_name: &str, val: u64) -> u64 {
        return self.map_id(self.name_id.get(from_name).unwrap(), self.name_id.get(to_name).unwrap(), val);
    }

    fn map_id(&self, from_id: &u64, to_id: &u64, val: u64) -> u64 {
        // println!("mapping {}-{} for {}", *from_id, *to_id, val);
        let mut current_src: &u64 = from_id;
        let mut v = val;
        loop {
            let (next, mapper) = self.maps.get(current_src).unwrap();
            v = mapper.map(v);
            if next == to_id {
                break;
            }
            current_src = next;
        }
        return v;
    }

    fn mapper_chain(&self, from_id: &u64, to_id: &u64) -> Vec<&RangeMap> {
        let mut out: Vec<&RangeMap> = vec![];
        let mut current_src: &u64 = from_id;
        loop {
            let (next, mapper) = self.maps.get(current_src).unwrap();
            out.push(mapper);
            if next == to_id {
                break;
            }
            current_src = next;
        }
        return out;
    }

    // problem: this consumes insane amounts of memory if returning a Vec<u64>, so need to provide a lazy iter instead
    fn expand_ranges(&self) -> Box<dyn Iterator<Item = SeedRange> + '_> {
        let starts = (0..self.seeds.len()).step_by(2).map(|i| self.seeds[i]);
        let ranges = (0..self.seeds.len()).step_by(2).map(|i| self.seeds[i+1]);
        let zipped = starts.zip(ranges);
        let iter_squared = zipped.map(|(from, range)| SeedRange::n(from as i64, (from as i64) + (range as i64) - 1, false));
        return Box::new(iter_squared);
    }
}

#[test]
fn almanac_loads() {
    let almanac = Almanac::load(&get_test_output());
    assert_eq!(81, almanac.map("seed", "soil", 79));
    assert_eq!(35, almanac.map("seed", "location", 13));
    assert_eq!(82, almanac.map("fertilizer", "temperature", 57));
}

#[derive(Debug)]
struct SeedRange {
    start: i64,
    end: i64,
    done_processing: bool
}
impl SeedRange {
    fn n(start: i64, end: i64, done_processing: bool) -> SeedRange {
        return SeedRange { start, end, done_processing };
    }

    fn offset(&mut self, offset: i64) {
        self.start += offset;
        self.end += offset;
    }
}

trait SplittableRange<T> {
    fn split_over(&self, a: T, b: T) -> Vec<Self> where Self: Sized;
}
impl SplittableRange<i64> for SeedRange {
    /// MUST be split from low range to high range
    /// takes out [a, b] inclusive
    fn split_over(&self, a: i64, b: i64) -> Vec<Self> {
        assert!(a < b);
        // everything within or before [a, b] (<= b) is considered processed for this step
        if (a > self.end && b > self.end) || (a < self.start && b < self.start) || (a <= self.start && b >= self.end) {
            return vec![SeedRange::n(self.start, self.end, self.end <= b)];
        } else if a <= self.start { // a   start  =b  end
            return vec![SeedRange::n(self.start, b, true), SeedRange::n(b+1, self.end, false)];
        } else if b >= self.end { // start  a=  end   b
            return vec![SeedRange::n(self.start, a-1, true), SeedRange::n(a, self.end, true)];
        } else if a > self.start && b < self.end { // start  a=  =b  end
            return vec![SeedRange::n(self.start, a-1, true), SeedRange::n(a, b, true), SeedRange::n(b+1, self.end, false)];
        } else {
            panic!("Impossible case reached, oops");
        }
    }
}
