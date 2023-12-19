use std::{fs, collections::HashMap};

use char_enum_impl::char_enum;

fn main() {
    println!("AOC 2023 Day 19");

    let contents = fs::read_to_string("src/bin/day19/input.txt").expect("Failed to read input");
    let sum = process(&contents);
    println!("Part 1: {}", sum);
}

fn process(data: &str) -> usize {
    let (workflows, xmases) = data.trim().split_once("\n\n").unwrap();
    let workflows_iter = workflows.trim().split("\n").map(|l| Workflow::parse(l));
    let workflows: HashMap<String, Workflow> = {
        let mut workflows: HashMap<String, Workflow> = HashMap::new();
        for (k, v) in workflows_iter {
            workflows.insert(k, v);
        }
        workflows
    };
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
            "R" => continue,
            _ => xmases.push((target.to_owned(), xmas))
        }
    }

    return sum;
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

#[test]
fn end_to_end_pipeline() {
    let test_data: &str = &get_test_data();
    assert_eq!(19114, process(test_data));
}
