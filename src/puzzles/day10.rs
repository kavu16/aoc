#[test]

fn test() {
    let data = String::from(
""
    );

    solve1(&data);

    solve2(&data);
}

// S = (20, 103)

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    let mut grid: Vec<Vec<char>> = vec![];
    for line in data {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    println!("Start is {}", grid[20][103]);

    println!("Part 1 = {}", res);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    for line in data {
        let mut oasis: Vec<i32> = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect();
        let mut final_sum: Vec<i32> = vec![];
        while !oasis.iter().all(|&n| n == 0) {
            let mut diffs: Vec<i32> = vec![];
            for i in 0..(oasis.len()-1) {
                diffs.push(oasis[i + 1] - oasis[i]);
            }
            let first = oasis[0];
            final_sum.push(first);
            oasis = diffs;
        }
        res += final_sum.iter().rev().fold(0, |a, &b| -(a-b));
    }
    println!("Part 2 = {}", res);
}