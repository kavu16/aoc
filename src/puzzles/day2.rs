#[test]

fn test() {
    solve1(&String::from(
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
        Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
        Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    ));
}

use::std::collections::HashMap;

pub fn solve1(data: &String) {
    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    let x: &[_] = &[':', ',', ';'];
    let mut result = 0;
    
    for line in data.lines() {
        let mut colors = HashMap::new();

        colors.insert("red", 0);
        colors.insert("green", 0);
        colors.insert("blue", 0);

        let bag = line.split_whitespace()
                                .map(|c| c.trim_end_matches(x))
                                .collect::<Vec<_>>();
        let id = bag[1].parse::<i32>().unwrap();
        for i in 1..bag.len()/2 {
            colors.insert(bag[2*i + 1], std::cmp::max(bag[2*i].parse::<i32>().unwrap(), *colors.get(bag[2*i + 1]).unwrap()));
        }

        if *colors.get("red").unwrap() > RED || *colors.get("green").unwrap() > GREEN || *colors.get("blue").unwrap() > BLUE {
            continue;
        } else {
            result += id;
        }
    }

    println!("Result = {result}");
}