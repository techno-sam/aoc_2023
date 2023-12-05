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

    let expanded = almanac.expand_ranges();
    println!("Expanded seed count: {}", expanded.len());
    let mut min2: Vec<(u64, u64)> = expanded.iter()
        .map(|seed| (*seed, almanac.map("seed", "location", *seed)))
        .collect();
    min2.sort_by(|(_s1, l1), (_s2, l2)| l1.partial_cmp(l2).unwrap());
    let (_, mininum_loc2) = min2[0];
    println!("Part 2 minimum location: {}", mininum_loc2);
}

struct RangeMapEntry {
    dest_start: u64,
    source_start: u64,
    range_length: u64,
}
impl RangeMapEntry {
    fn load(entry: &str) -> RangeMapEntry {
        let mut parts = entry.trim().split(" ").filter(|s| !s.is_empty());
        let dst = parts.next().unwrap().parse::<u64>().unwrap();
        let src = parts.next().unwrap().parse::<u64>().unwrap();
        let range_length = parts.next().unwrap().parse::<u64>().unwrap();
        return RangeMapEntry { dest_start: dst, source_start: src, range_length };
    }

    fn map(&self, val: u64) -> Option<u64> {
        if val < self.source_start || val >= self.source_start + self.range_length {
            return None;
        }
        let mut v = val as i64;
        v -= self.source_start as i64;
        v += self.dest_start as i64;
        return Some(v as u64);
    }
}

struct RangeMap {
    name: String,
    entries: Vec<RangeMapEntry>
}
impl RangeMap {
    fn load(name: &str, entries: &str) -> RangeMap {
        let ent: Vec<RangeMapEntry> = entries.split("\n")
            .filter(|s| !s.is_empty())
            .map(|s| RangeMapEntry::load(s))
            .collect();
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
}

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
    maps: HashMap<String, (String, RangeMap)>
}
impl Almanac {
    fn load(data: &str) -> Almanac {
        let (seeds, maps_str) = data.trim().split_once("\n\n").unwrap();
        let maps = maps_str.split("\n\n");
        let mut all_maps: HashMap<String, (String, RangeMap)> = HashMap::new();
        for map in maps {
            let (title, entries) = map.split_once(" map:\n").unwrap();
            let (from, to) = title.split_once("-to-").unwrap();
            let map = RangeMap::load(title, entries);
            all_maps.insert(from.to_string(), (to.to_string(), map));
        }
        let seed_vals: Vec<u64> = seeds.split(" ")
            .filter(|s| !s.is_empty())
            .filter(|s| s != &"seeds:")
            .map(|s| s.parse::<u64>().expect(&("Failed to parse [".to_owned()+s+"]")))
            .collect();
        return Almanac { seeds: seed_vals, maps: all_maps };
    }

    fn map(&self, from_name: &str, to_name: &str, val: u64) -> u64 {
        let mut current_src: &str = from_name;
        let mut v = val;
        loop {
            let (next, mapper) = self.maps.get(current_src).unwrap();
            v = mapper.map(v);
            if next == to_name {
                break;
            }
            current_src = next;
        }
        return v;
    }

    fn expand_ranges(&self) -> Vec<u64> {
        let mut out: Vec<u64> = vec![];

        for i in (0..self.seeds.len()).step_by(2) {
            let from = self.seeds[i];
            let range = self.seeds[i+1];
            for v in from..(from+range) {
                out.push(v);
            }
        }

        println!("\tExpanded range of length: {}", out.len());

        return out;
    }
}

#[test]
fn almanac_loads() {
    let almanac = Almanac::load(&get_test_output());
    assert_eq!(81, almanac.map("seed", "soil", 79));
    assert_eq!(35, almanac.map("seed", "location", 13));
    assert_eq!(82, almanac.map("fertilizer", "temperature", 57));
}
