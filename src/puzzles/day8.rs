#[test]

fn test() {
    let data = String::from(
"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    );

    solve1(&data);

    solve2(&data);
}

use std::collections::HashMap;
use num::integer::lcm;

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let mut directions = data[0].chars().cycle();
    let mut step = 0;

    for line in data[2..].iter() {
        let (key, line) = line.split_once('=').unwrap();
        let values: Vec<&str> = line.trim().split(',').collect();
        let lval = values[0].trim_start_matches('(').trim();
        let rval = values[1].trim_end_matches(')').trim();
        map.insert(key.trim(), (lval, rval));
    }

    let mut pos = "AAA";
    while let Some(d) = directions.next() {
        match d {
            'L' => {
                pos = map.get(pos).unwrap().0;
            }
            'R' => {
                pos = map.get(pos).unwrap().1;
            }
            _ => break
        }
        step += 1;
        if pos == "ZZZ" {break;}
    }

    println!("Part 1 = {}", step);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut map: HashMap<&str, (&str, &str)> = HashMap::new();
    let d_start = data[0];

    for line in data[2..].iter() {
        let (key, line) = line.split_once('=').unwrap();
        let values: Vec<&str> = line.trim().split(',').collect();
        let lval = values[0].trim_start_matches('(').trim();
        let rval = values[1].trim_end_matches(')').trim();
        map.insert(key.trim(), (lval, rval));
    }

    let starts: Vec<&&str> = map.keys().filter(|&&k| k.chars().rev().next().unwrap() == 'A').collect();
    let mut ends: Vec<u64> = vec![];

    for p in starts {
        let mut pos = *p;
        let mut step = 0;
        let mut directions = d_start.chars().cycle();
        while let Some(d) = directions.next() {
            match d {
                'L' => {
                    pos = map.get(pos).unwrap().0;
                }
                'R' => {
                    pos = map.get(pos).unwrap().1;
                }
                _ => break
            }
            step += 1;
            if pos.chars().rev().next().unwrap() == 'Z' {
                ends.push(step);
                break;
            }
        }
    }
    let final_step = ends.iter().fold(1, |a, &b| lcm(a, b));

    println!("Part 2 = {}", final_step);
}