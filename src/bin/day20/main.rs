use std::hash::{Hasher, Hash};
use std::{fs, collections::{HashMap, VecDeque, HashSet, hash_map::DefaultHasher}};

use char_enum_impl::data_enum;

fn main() {
    println!("AOC 2023 Day 20");

    let contents = fs::read_to_string("src/bin/day20/input.txt").expect("Failed to read input");

    let mut layout = Layout::load(&contents);
    let mut layout2 = layout.clone();
    let part1 = layout.thousand_product();

    println!("Part 1: {}", part1);

    for i in 1..=1_000_000_usize {
        println!("Starting iteration {}", i);
        layout2.thousand_product();
    }
}

#[derive(Clone, Debug)]
enum Module {
    /// (false - off, true - on)
    FlipFlop(bool),
    /// input -> off/on
    Conjunction(HashMap<String, bool>),
    Broadcaster,
    /// keeps track of received signals
    Output(Vec<bool>),
    /// does literally nothing
    Drain,
}
impl Module {
    fn parse(data: &str) -> (String, Module) {
        if data == "broadcaster" {
            return (data.to_owned(), Module::Broadcaster);
        }
        if data == "output" {
            return (data.to_owned(), Module::Output(vec![]));
        }
        if data.starts_with("%") {
            return (data[1..].to_owned(), Module::FlipFlop(false));
        }
        if data.starts_with("&") {
            return (data[1..].to_owned(), Module::Conjunction(HashMap::new()));
        }
        panic!("Unable to parse module");
    }
}
impl Hash for Module {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Module::FlipFlop(b) => {
                b.hash(state);
            },
            Module::Conjunction(states) => {
                let mut keys: Vec<String> = states.keys().map(|s| s.to_owned()).collect();
                keys.sort();
                for key in keys {
                    key.hash(state);
                    states.get(&key).unwrap().hash(state);
                }
            },
            _ => panic!("Hash not supported for {:?}", self)
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[data_enum(bool)]
enum Pulse {
    Low = false,
    High = true
}
impl Into<Pulse> for bool {
    fn into(self) -> Pulse {
        if self {
            return Pulse::High;
        } else {
            return Pulse::Low;
        }
    }
}

/*
 * note: the names of these may differ for your input, make sure to adjust [Layout::feeder_to_idx]
 * rx will get a low pulse when &gq has gotten high pulses from all of its inputs
 * (&xj, &qs, &kz, &km)
 */

#[derive(Clone)]
struct Layout {
    modules: HashMap<String, (Module, Vec<String>)>,
    all_stateful_modules: Vec<String>,
    previous_hashes: HashSet<u64>,
    iters: usize,
    intervals: [(Option<usize>, Option<usize>); 4]
}
impl Layout {
    /// convert input to 'gq' to an index in the list
    fn feeder_to_idx(feeder: &String) -> usize {
        let feeder: &str = &*feeder;
        match feeder {
            "xj" => 0,
            "qs" => 1,
            "kz" => 2,
            "km" => 3,
            _ => panic!("Invalid feeder")
        }
    }

    fn hash_me(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        for module in &self.all_stateful_modules {
            let (module, _) = self.modules.get(module).unwrap();
            module.hash(&mut hasher);
        }
        hasher.finish()
    }

    fn load(data: &str) -> Layout {
        let mut modules: HashMap<String, (Module, Vec<String>)> = HashMap::new();
        let mut output_found = false;

        let lines = data.trim().split("\n")
            .map(|l| {
                let (src, dst) = l.split_once(" -> ").unwrap();
                let (src_name, src_module) = Module::parse(src);
                let dst: Vec<String> = dst.split(", ").map(|s| s.to_owned()).collect();
                if dst.contains(&"output".to_owned()) {
                    output_found = true;
                }
                return (src_name, (src_module, dst));
            });
        lines.for_each(|(k, v)| {modules.insert(k, v);});
        if output_found {
            modules.insert("output".to_owned(), (Module::parse("output").1, vec![]));
        }

        let mut stateful_modules: Vec<String> = vec![];

        let mut conjunctions: HashSet<String> = HashSet::new();
        for (name, (module, _)) in &modules {
            match module {
                Module::FlipFlop(_) => {
                    stateful_modules.push(name.to_owned());
                },
                Module::Conjunction(_) => {
                    stateful_modules.push(name.to_owned());
                    conjunctions.insert(name.to_owned());
                },
                _ => {}
            }
        }

        let imu_modules = &modules.clone();

        for (name, (_, targets)) in imu_modules {
            for target in targets {
                if conjunctions.contains(target) {
                    if let Module::Conjunction(states) = &mut modules.get_mut(target).unwrap().0 {
                        states.insert(name.to_owned(), false);
                    }
                }
            }
        }

        return Layout {
            modules,
            all_stateful_modules: stateful_modules,
            previous_hashes: HashSet::new(),
            iters: 0,
            intervals: [(None, None); 4]
        };
    }

    /// return: (output_signals, low_pulses, high_pulses)
    fn press_once(&mut self) -> (Vec<bool>, usize, usize) {
        /*let hash = self.hash_me();
        if self.previous_hashes.contains(&hash) {
            panic!("Cycle detected!!!");
        }
        self.previous_hashes.insert(hash);*/

        // Push to back, pop from front
        let mut pulses: VecDeque<(String, String, Pulse)> = VecDeque::new();
        let mut low_pulse_count: usize = 0;
        let mut high_pulse_count: usize = 0;
        pulses.push_back(("button".to_owned(), "broadcaster".to_owned(), Pulse::Low));

        while pulses.len() > 0 {
            let (src, current, pulse) = pulses.pop_front().unwrap();

            if pulse.value() {
                high_pulse_count += 1;
            } else {
                low_pulse_count += 1;
            }

            if !self.modules.contains_key(&current) {
                println!("Module {} does not seem to exist, adding it", current);
                self.modules.insert(current, (Module::Drain, vec![]));
                continue;
            }

            if current == "rx" && !pulse.value() {
                panic!("Low pulse to rx!");
            }

            let (module, next_targets) = self.modules.get_mut(&current).expect(&format!("Module '{}' not found", current));

            #[cfg(test)]
            println!("{} {:?} -> {}", src, pulse, current);

            match module {
                Module::FlipFlop(state) => {
                    if let Pulse::High = pulse {
                        continue;
                    }
                    *state = !*state;
                    for next_target in next_targets {
                        pulses.push_back((current.to_owned(), next_target.to_owned(), (*state).into()));
                    }
                },
                Module::Conjunction(states) => {
                    let old = states.get(&src).unwrap();
                    if current == "gq" {
                        if pulse.value() != *old {
                            println!("\n\n== iter {} gq got a {:?} pulse from {}", self.iters, pulse, src);
                        }
                        if pulse.value() {
                            let idx = Layout::feeder_to_idx(&src);
                            let (first, second) = &mut self.intervals[idx];
                            if let None = first {
                                *first = Some(self.iters);
                            } else if let None = second {
                                *second = Some(self.iters);

                                // go through and check that all are filled in
                                let all_filled = self.intervals.iter().all(|(a, b)| {
                                    if let None = a {
                                        return false;
                                    }
                                    if let None = b {
                                        return false;
                                    }
                                    return true;
                                });
                                if all_filled {
                                    let mut product: usize = 1;
                                    for (a, b) in self.intervals {
                                        let start = a.unwrap();
                                        let cycle_len = b.unwrap() - start;
                                        println!("start counting from 0: {}, cycle: {}", start, cycle_len);
                                        product *= cycle_len; // magic works
                                    }
                                    println!("Part 2: {}", product);
                                    panic!("We're done!");
                                }
                            }
                        }
                    }
                    states.insert(src, pulse.value());
                    let next_pulse: Pulse = (!states.iter()
                        .map(|(_, b)| *b)
                        .reduce(|acc, e| acc && e)
                        .unwrap()).into();

                    for next_target in next_targets {
                        pulses.push_back((current.to_owned(), next_target.to_owned(), next_pulse));
                    }
                },
                Module::Broadcaster => { // literally just a relay
                    for next_target in next_targets {
                        pulses.push_back((current.to_owned(), next_target.to_owned(), pulse));
                    }
                },
                Module::Output(signals) => { // drain
                    signals.push(pulse.value());
                },
                Module::Drain => {}
            };
        }

        self.iters += 1;

        if let Some((Module::Output(signals), _)) = self.modules.get("output") {
            return (signals.clone(), low_pulse_count, high_pulse_count);
        } else {
            return (vec![], low_pulse_count, high_pulse_count);
        }
    }

    fn thousand_product(&mut self) -> usize {
        let mut low_sum: usize = 0;
        let mut high_sum: usize = 0;
        for _ in 0..1000 {
            let (_, low, high) = self.press_once();
            low_sum += low;
            high_sum += high;
        }
        return low_sum * high_sum;
    }
}

#[test]
fn simple_network() {
    let mut layout: Layout = Layout::load("
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
");
    let mut layout2 = layout.clone();
    let (_, low, high) = layout.press_once();
    assert_eq!(8, low);
    assert_eq!(4, high);
    assert_eq!(32000000, layout2.thousand_product());
}

#[test]
fn complex_network() {
    let mut layout: Layout = Layout::load("
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
");
    let mut layout2 = layout.clone();

    let (_, low, high) = layout.press_once();
    assert_eq!(4, low, "1 Press, low");
    assert_eq!(4, high, "1 Press, high");


    assert_eq!(11687500, layout2.thousand_product(), "All the counting");
}
