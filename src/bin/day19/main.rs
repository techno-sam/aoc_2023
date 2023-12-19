use std::{fs, collections::HashMap};

use char_enum_impl::char_enum;

fn main() {
    println!("AOC 2023 Day 19");

    let contents = fs::read_to_string("src/bin/day19/input.txt").expect("Failed to read input");
    let sum = process(&contents);
    println!("Part 1: {}", sum);

    let sum2 = process_hyper(&contents);
    println!("Part 2: {}", sum2);
}

fn parse_workflows(data: &str) -> HashMap<String, Workflow> {
    let workflows_iter = data.trim().split("\n").map(|l| Workflow::parse(l));
    let mut workflows: HashMap<String, Workflow> = HashMap::new();
    for (k, v) in workflows_iter {
        workflows.insert(k, v);
    }
    return workflows;
}

fn process(data: &str) -> usize {
    let (workflows, xmases) = data.trim().split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);
    let mut xmases: Vec<(String, Xmas)> = xmases.trim()
        .split("\n")
        .map(|l| ("in".to_owned(), Xmas::parse(l)))
        .collect();

    let mut sum: usize = 0;

    while xmases.len() > 0 {
        let (workflow_id, xmas) = xmases.pop().unwrap();
        let workflow = workflows.get(&workflow_id).unwrap();
        let target: &str = &workflow.process(&xmas);
        match target {
            "A" => sum += xmas.rating(),
            "R" => {},
            _ => xmases.push((target.to_owned(), xmas))
        }
    }

    return sum;
}

fn process_hyper(data: &str) -> usize {
    let mut sum: usize = 0;

    let (workflows, _) = data.trim().split_once("\n\n").unwrap();
    let workflows = parse_workflows(workflows);

    let mut cubes: Vec<(String, Hypercube)> = vec![("in".to_owned(), Hypercube::initial())];

    while cubes.len() > 0 {
        let (workflow_id, cube) = cubes.pop().unwrap();
        let mut cube = cube;
        let workflow = workflows.get(&workflow_id).unwrap();

        // steps handling:
        // Unconditional: whole cube goes to target
        // GreaterThan(var, bound, target): split(var, bound), [1] goes to target, [0] becomes the
        //   new value of cube
        // LessThan(var, bound, target): split(var, bound-1), [0] goes to target, [1] becomes the
        //   new value of cube

        'Steps: for step in &workflow.steps {
            match step {
                Step::Unconditional(target) => {
                    let target: &str = target;
                    match target {
                        "A" => sum += cube.volume(),
                        "R" => {},
                        _ => cubes.push((target.to_owned(), cube))
                    }
                    break 'Steps;
                },
                Step::GreaterThan(var, bound, target) => {
                    let target: &str = target;
                    let (fail, pass) = cube.split(var, *bound);
                    cube = fail;
                    match target {
                        "A" => sum += pass.volume(),
                        "R" => {},
                        _ => cubes.push((target.to_owned(), pass))
                    }
                },
                Step::LessThan(var, bound, target) => {
                    let target: &str = target;
                    let (pass, fail) = cube.split(var, *bound - 1);
                    cube = fail;
                    match target {
                        "A" => sum += pass.volume(),
                        "R" => {},
                        _ => cubes.push((target.to_owned(), pass))
                    }
                }
            }
        }
    }
    return sum;
}

#[derive(Debug)]
struct Hypercube {
    x0: usize,
    x1: usize,

    m0: usize,
    m1: usize,

    a0: usize,
    a1: usize,

    s0: usize,
    s1: usize
}
impl Hypercube {
    fn initial() -> Hypercube {
        return Hypercube { x0: 1, x1: 4000, m0: 1, m1: 4000, a0: 1, a1: 4000, s0: 1, s1: 4000 };
    }

    /// \[v0, v1\] over v_m => \[v0, vm\], \[vm+1, v0\]
    /// So, for a (var > #n) comp, you want split(var_axis, #n)\[1\] for match and \[0\] for non-match
    /// For a (var < #n) comp, you want split(var_axis, #n - 1)\[0\] for match and \[1\] for non-match
    fn split(self, axis: &Variable, point: usize) -> (Hypercube, Hypercube) {
        match axis {
            Variable::X => (Hypercube {x0: self.x0, x1: point, ..self}, Hypercube {x0: point+1, x1: self.x1, ..self}),
            Variable::M => (Hypercube {m0: self.m0, m1: point, ..self}, Hypercube {m0: point+1, m1: self.m1, ..self}),
            Variable::A => (Hypercube {a0: self.a0, a1: point, ..self}, Hypercube {a0: point+1, a1: self.a1, ..self}),
            Variable::S => (Hypercube {s0: self.s0, s1: point, ..self}, Hypercube {s0: point+1, s1: self.s1, ..self}),
        }
    }

    #[inline]
    fn volume(self) -> usize {
        let out = (self.x1 - self.x0 + 1) * (self.m1 - self.m0 + 1) * (self.a1 - self.a0 + 1) * (self.s1 - self.s0 + 1);
        #[cfg(test)]
        println!("Calculate volume of {:?} to be {}", self, out);
        return out;
    }
}

#[char_enum]
enum Variable {
    X = 'x',
    M = 'm',
    A = 'a',
    S = 's'
}

enum Step {
    /// _ => target
    Unconditional(String),
    /// variable > boundary => target
    GreaterThan(Variable, usize, String),
    /// variable < boundary => target
    LessThan(Variable, usize, String),
}
impl Step {
    fn parse(data: &str) -> Step {
        if data.contains("<") {
            let (var, bound_target) = data.split_once("<").unwrap();
            let var = Variable::decode(var.chars().nth(0).unwrap());
            let (bound, target) = bound_target.split_once(":").unwrap();
            let bound = bound.parse::<usize>().unwrap();
            return Step::LessThan(var, bound, target.to_owned());
        } else if data.contains(">") {
            let (var, bound_target) = data.split_once(">").unwrap();
            let var = Variable::decode(var.chars().nth(0).unwrap());
            let (bound, target) = bound_target.split_once(":").unwrap();
            let bound = bound.parse::<usize>().unwrap();
            return Step::GreaterThan(var, bound, target.to_owned());
        }
        return Step::Unconditional(data.to_owned());
    }
}

struct Workflow {
    steps: Vec<Step>
}
impl Workflow {
    fn parse(line: &str) -> (String, Workflow) {
        let line = line.strip_suffix("}").unwrap();
        let (id, steps_str) = line.split_once("{").unwrap();

        let steps: Vec<Step> = steps_str.split(",").map(|s| Step::parse(s)).collect();

        return (id.to_owned(), Workflow { steps });
    }

    /// returns target workflow, including "A" or "R"
    fn process(&self, xmas: &Xmas) -> String {
        for step in &self.steps {
            match step {
                Step::Unconditional(target) => return target.to_owned(),
                Step::GreaterThan(var, boundary, target) => {
                    if xmas.get(var) > *boundary {
                        return target.to_owned();
                    }
                },
                Step::LessThan(var, boundary, target) => {
                    if xmas.get(var) < *boundary {
                        return target.to_owned();
                    }
                }
            }
        }
        panic!("No steps matched, oops");
    }
}

struct Xmas {
    x: usize,
    m: usize,
    a: usize,
    s: usize
}
impl Xmas {
    fn parse(line: &str) -> Xmas {
        let line = line.trim()
            .strip_prefix("{x=").unwrap()
            .strip_suffix("}").unwrap();
        // 787,m=2655,a=1222,s=2876
        let (x, mas) = line.split_once(",m=").unwrap();
        let x = x.parse().unwrap();

        let (m, as_) = mas.split_once(",a=").unwrap();
        let m = m.parse().unwrap();

        let (a, s) = as_.split_once(",s=").unwrap();
        let a = a.parse().unwrap();
        let s = s.parse().unwrap();

        return Xmas { x, m, a, s };
    }

    fn get(&self, var: &Variable) -> usize {
        match var {
            Variable::X => self.x,
            Variable::M => self.m,
            Variable::A => self.a,
            Variable::S => self.s
        }
    }

    #[inline(always)]
    fn rating(&self) -> usize {
        return self.x + self.m + self.a + self.s;
    }
}

#[allow(dead_code)]
fn get_test_data() -> String {
    return "
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
".to_owned();
}

#[allow(dead_code)]
fn get_simple_test_data() -> String {
    return "
in{x<2000:foo,A}
foo{s<2000:R,A}

{x=12,m=13,a=14,s=15}
".to_owned();
}

#[test]
fn end_to_end_pipeline() {
    let test_data: &str = &get_test_data();
    assert_eq!(19114, process(test_data));
}

#[test]
fn end_to_end_simple_hyper_pipeline() {
    let test_data: &str = &get_simple_test_data();
    assert_eq!(256000000000000_usize, 4000*4000*4000*4000, "Sanity check 1");
    assert_eq!(256000000000000_usize, Hypercube::initial().volume(), "Sanity check 2");
    println!("\n\nDone with sanity checks");
    let res: usize = (2001*4000*4000*4000) + (1999*4000*4000*2001);
    assert_eq!(res, process_hyper(test_data));
}

#[test]
fn end_to_end_hyper_pipeline() {
    let test_data: &str = &get_test_data();
    assert_eq!(167409079868000, process_hyper(test_data));
}
