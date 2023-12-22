use std::collections::{HashSet, HashMap};

#[test]

fn test() {
    let data = String::from(
"2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    );

    assert_eq!(solve1(&data), 46);

    assert_eq!(solve2(&data), 51);
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Direction {
    Right,
    Left, 
    Up,
    Down,
}

fn djikstra(r: usize, c: usize, grid: &Vec<Vec<usize>>, visited: &mut HashSet<(usize,usize)>) {
    return;
}

pub fn solve1(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in data {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>());
    }

    let res = 0;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let data = data.lines().collect::<Vec<_>>();
    let mut grid: Vec<Vec<usize>> = Vec::new();
    for line in data {
        grid.push(line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect::<Vec<usize>>());
    }

    let res = 0;
    println!("Part 2 = {}", res);
    res
}
