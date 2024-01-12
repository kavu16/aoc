// use std::collections::{VecDeque, HashSet};

#[test]

fn test() {
    let example1 = String::from(
"1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
    );

    assert_eq!(solve1(&example1), 0);
    assert_eq!(solve2(&example1), 0);
}

pub fn solve1(_data: &String) -> usize {
    let res = 0;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(_data: &String) -> usize {
    let res = 0;
    println!("Part 2 = {}", res);
    res
}