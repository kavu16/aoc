use std::collections::{HashMap, HashSet};

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

    let mut grid: Vec<Vec<char>> = vec![];
    for line in data {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let pipes: HashMap<char, ((i32,i32),(i32,i32))> = HashMap::from([
        ('F', ((1,0), (0,1))),
        ('7', ((1,0), (0,-1))),
        ('J', ((0,-1), (-1,0))),
        ('L', ((0,1), (-1,0))),
        ('|', ((1,0), (-1,0))),
        ('-', ((0,-1), (0,1)))
    ]);

    let mut visited: HashSet<(i32,i32)> = HashSet::new();
    let start = (20,103);
    visited.insert(start);

    let mut curr: ((i32, i32), (i32, i32)) = ((20,102), (21,103));
    let mut step = 1;

    while curr.0 != curr.1 {
        let (curr_0_a, curr_0_b) = pipes.get(&grid[curr.0.0 as usize][curr.0.1 as usize]).unwrap();
        if visited.contains(&(curr.0.0 + curr_0_a.0, curr.0.1 + curr_0_a.1)) {
            curr.0.0 += curr_0_b.0;
            curr.0.1 += curr_0_b.1;
            visited.insert((curr.0.0, curr.0.1));
        } else {
            curr.0.0 += curr_0_a.0;
            curr.0.1 += curr_0_a.1;
            visited.insert((curr.0.0, curr.0.1));
        }
        let (curr_1_a, curr_1_b) = pipes.get(&grid[curr.1.0 as usize][curr.1.1 as usize]).unwrap();
        if visited.contains(&(curr.1.0 + curr_1_a.0, curr.1.1 + curr_1_a.1)) {
            curr.1.0 += curr_1_b.0;
            curr.1.1 += curr_1_b.1;
            visited.insert((curr.1.0, curr.1.1));
        } else {
            curr.1.0 += curr_1_a.0;
            curr.1.1 += curr_1_a.1;
            visited.insert((curr.1.0, curr.1.1));
        }
        step += 1;
    }
    println!("Part 1 = {}", step);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<char>> = vec![];
    for line in data {
        grid.push(line.chars().collect::<Vec<char>>());
    }

    let pipes: HashMap<char, ((i32,i32),(i32,i32))> = HashMap::from([
        ('F', ((1,0), (0,1))),
        ('7', ((1,0), (0,-1))),
        ('J', ((0,-1), (-1,0))),
        ('L', ((0,1), (-1,0))),
        ('|', ((1,0), (-1,0))),
        ('-', ((0,-1), (0,1)))
    ]);

    let mut visited: Vec<(i32,i32)> = Vec::new();
    let start = (20, 103);
    visited.push(start);

    let mut curr: (i32, i32) = (20,102); //(21,103));
    let mut prev = start;
    while curr != start {
        let (next_a, next_b) = pipes.get(&grid[curr.0 as usize][curr.1 as usize]).unwrap();
        if prev == (curr.0 + next_a.0, curr.1 + next_a.1) {
            prev = curr;
            visited.push(curr);
            curr.0 += next_b.0;
            curr.1 += next_b.1;
        } else {
            prev = curr;
            visited.push(curr);
            curr.0 += next_a.0;
            curr.1 += next_a.1;
        }
    }

    let b = visited.len();
    visited.push(start);
    let mut plus_lace = 0;
    let mut minus_lace = 0;

    for i in 0..b {
        plus_lace += visited[i].0*visited[i+1].1;
        minus_lace += visited[i].1*visited[i+1].0;
    }
    let area = (plus_lace - minus_lace) / 2;

    let res = (area + 1) - (b as i32)/2;
    println!("Part 2 = {}", res);
}