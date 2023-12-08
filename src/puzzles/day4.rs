#[test]

fn test() {
    solve1(&String::from(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
    ));

    solve2(&String::from(
"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11"
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