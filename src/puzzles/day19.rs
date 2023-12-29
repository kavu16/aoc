use std::collections::{HashMap, HashSet};

#[test]

fn test() {
    let data = String::from(
"px{a<2006:qkq,m>2090:A,rfg}
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
{x=2127,m=1623,a=2188,s=1013}"
    );

    assert_eq!(solve1(&data), 19_114);

    assert_eq!(solve2(&data), 167_409_079_868_000);
}

fn accept_val(gear: &HashMap<char, u32>, 
              workflows: &HashMap<String, Vec<String>>, 
              mut curr: String) -> u32 {
    let mut res = 0;
    'f: while let Some(currflow) = workflows.get(&curr) {
        for flow in currflow {
            if flow == "R" {
                return 0;
            } else if flow == "A" {
                for (_k, v) in gear {
                    res += v;
                }
                return res;
            } else if flow.contains(":") {
                let (cond, dest) = flow.split_once(":").unwrap();
                let dest = String::from(dest);
                if cond.contains("<") {
                    let (key, val) = cond.split_once("<").unwrap();
                    let key = key.chars().next().unwrap();
                    let val = val.parse::<u32>().unwrap();
                    if gear.get(&key).unwrap() < &val {
                        match dest.as_str() {
                            "R" => return 0,
                            "A" => {
                                for (_k, v) in gear {
                                    res += v;
                                }
                                return res;
                            }
                            _ => {
                                curr = dest;
                                continue 'f;
                            }
                        }
                    }
                } else if cond.contains(">") {
                    let (key, val) = cond.split_once(">").unwrap();
                    let key = key.chars().next().unwrap();
                    let val = val.parse::<u32>().unwrap();
                    if gear.get(&key).unwrap() > &val {
                        match dest.as_str() {
                            "R" => return 0,
                            "A" => {
                                for (_k, v) in gear {
                                    res += v;
                                }
                                return res;
                            }
                            _ => {
                                curr = dest;
                                continue 'f;
                            }
                        }
                    }
                }
            } else {
                curr = String::from(flow);
            }
        }
    }
    res
}

pub fn solve1(data: &String) -> u32 {
    let (maps, gears) = data.split_once("\n\n").unwrap();
    let mut workflows = HashMap::<String, Vec<String>>::new();
    let mut gearpile = Vec::<HashMap<char, u32>>::new();

    for line in maps.lines() {
        let (key, tail) = line.split_once("{").unwrap();
        let key = String::from(key);
        let tail = tail.trim_end_matches("}");
        let mut flow = Vec::<String>::new();
        tail.split(",").for_each(|c| {
            flow.push(String::from(c));
        });
        workflows.insert(key, flow);
    }

    for gear in gears.lines() {
        let ratings = gear.trim_start_matches("{").trim_end_matches("}");
        let mut gear = HashMap::<char, u32>::new();
        ratings.split(",").for_each(|r| {
            let (k, v) = r.split_once("=").unwrap();
            let k = k.chars().next().unwrap();
            gear.insert(k, v.parse::<u32>().unwrap());
        });
        gearpile.push(gear);
    }

    let mut res = 0;
    for gear in gearpile {
        let curr = String::from("in");
        res += accept_val(&gear, &workflows, curr);
    }

    println!("Part 1 = {}", res);
    res
}

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Combo {
    x: (u16, u16),
    m: (u16, u16),
    a: (u16, u16),
    s: (u16, u16),
}

fn accept_range(accepted: &mut HashSet<Combo>,
                ranges: Combo,
                curr: String,
                workflows: &HashMap<String, Vec<String>>) {
    let currflow = workflows.get(&curr).unwrap();
    let mut curr_ranges = ranges;

    for flow in currflow { 
        if flow == "R" {
            return;
        } else if flow == "A" {
            accepted.insert(curr_ranges);
        } else if flow.contains(":") {
            let mut new_ranges = curr_ranges;
            let (cond, dest) = flow.split_once(":").unwrap();
            let dest = String::from(dest);
            if cond.contains("<") {
                let (key, val) = cond.split_once("<").unwrap();
                let key = String::from(key);
                let val = val.parse::<u16>().unwrap();
                match dest.as_str() {
                    "R" => (),
                    "A" => {
                        match key.as_str() {
                            "x" => new_ranges.x.1 = val - 1,
                            "m" => new_ranges.m.1 = val - 1,
                            "a" => new_ranges.a.1 = val - 1,
                            "s" => new_ranges.s.1 = val - 1,
                            _ => println!("Unrecognized Pattern"),
                        }
                        accepted.insert(new_ranges);
                    }
                    _ => {
                        match key.as_str() {
                            "x" => new_ranges.x.1 = val - 1,
                            "m" => new_ranges.m.1 = val - 1,
                            "a" => new_ranges.a.1 = val - 1,
                            "s" => new_ranges.s.1 = val - 1,
                            _ => println!("Unrecognized Pattern"),
                        }
                        accept_range(accepted, new_ranges, dest, workflows);
                    }
                } 
                match key.as_str() {
                    "x" => curr_ranges.x.0 = val,
                    "m" => curr_ranges.m.0 = val,
                    "a" => curr_ranges.a.0 = val,
                    "s" => curr_ranges.s.0 = val,
                    _ => println!("Unrecognized Pattern"),
                }
            } else if cond.contains(">") {
                let (key, val) = cond.split_once(">").unwrap();
                let key = String::from(key);
                let val = val.parse::<u16>().unwrap();
                match dest.as_str() {
                    "R" => (),
                    "A" => {
                        match key.as_str() {
                            "x" => new_ranges.x.0 = val + 1,
                            "m" => new_ranges.m.0 = val + 1,
                            "a" => new_ranges.a.0 = val + 1,
                            "s" => new_ranges.s.0 = val + 1,
                            _ => println!("Unrecognized Pattern"),
                        }
                        accepted.insert(new_ranges);
                    }
                    _ => {
                        match key.as_str() {
                            "x" => new_ranges.x.0 = val + 1,
                            "m" => new_ranges.m.0 = val + 1,
                            "a" => new_ranges.a.0 = val + 1,
                            "s" => new_ranges.s.0 = val + 1,
                            _ => println!("Unrecognized Pattern"),
                        }
                        accept_range(accepted, new_ranges, dest, workflows);
                    }
                } 
                match key.as_str() {
                    "x" => curr_ranges.x.1 = val,
                    "m" => curr_ranges.m.1 = val,
                    "a" => curr_ranges.a.1 = val,
                    "s" => curr_ranges.s.1 = val,
                    _ => println!("Unrecognized Pattern"),
                }
            }
        } else {
            let dest = String::from(flow);
            accept_range(accepted, curr_ranges, dest, workflows);
        }
    }
}

pub fn solve2(data: &String) -> u128 {
    let (maps, _gears) = data.split_once("\n\n").unwrap();
    let mut workflows = HashMap::<String, Vec<String>>::new();

    for line in maps.lines() {
        let (key, tail) = line.split_once("{").unwrap();
        let key = String::from(key);
        let tail = tail.trim_end_matches("}");
        let mut flow = Vec::<String>::new();
        tail.split(",").for_each(|c| {
            flow.push(String::from(c));
        });
        workflows.insert(key, flow);
    }

    let ranges = Combo { x: (1, 4000), m: (1, 4000), a: (1, 4000), s: (1, 4000) };
    let mut accepted = HashSet::<Combo>::new();

    accept_range(&mut accepted, ranges, String::from("in"), &workflows);

    let mut res = 0;
    for Combo { x: (xmin, xmax), 
                m: (mmin, mmax),
                a: (amin, amax),
                s: (smin, smax)} in accepted {
        res += (xmax as u128 - xmin as u128 + 1) * 
                (mmax as u128 - mmin as u128 + 1) * 
                (amax as u128 - amin as u128 + 1) * 
                (smax as u128 - smin as u128 + 1);
    }
    println!("Part 2 = {}", res);
    res
}
