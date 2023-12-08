#[test]

fn test() {
    solve1(&String::from(
""
    ));

    solve2(&String::from(
""
    ));
}

use::std::collections::HashSet;

pub fn solve1(data: &String) {
    let mut score = 0;

    for line in data.lines() {
        let nums: Vec<&str> = line.split('|').collect();
        let winners: HashSet<i32> = nums[0].split_whitespace().filter(|c| c.parse::<i32>().is_ok()).map(|n| n.parse::<i32>().unwrap()).collect();
        let my_nums: HashSet<i32> = nums[1].split_whitespace().filter(|c| c.parse::<i32>().is_ok()).map(|n| n.parse::<i32>().unwrap()).collect();

        let p = winners.intersection(&my_nums).count();
        if p == 0 {continue;}
        score += 2i32.pow(p as u32 - 1);
    }

    println!("Part 1 = {}", score);
}

pub fn solve2(data: &String) {
    let mut tickets = vec![1; data.lines().count()];

    for (index, line) in data.lines().enumerate() {
        let nums: Vec<&str> = line.split('|').collect();
        let winners: HashSet<i32> = nums[0].split_whitespace().filter(|c| c.parse::<i32>().is_ok()).map(|n| n.parse::<i32>().unwrap()).collect();
        let my_nums: HashSet<i32> = nums[1].split_whitespace().filter(|c| c.parse::<i32>().is_ok()).map(|n| n.parse::<i32>().unwrap()).collect();

        let m = winners.intersection(&my_nums).count();

        for i in (index+1)..(index+1+m) {
            tickets[i] += tickets[index];
        }
    }

    println!("Part 2 = {}", tickets.iter().sum::<i32>());
}