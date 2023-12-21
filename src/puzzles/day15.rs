use std::collections::HashMap;

#[test]

fn test() {
    let data = String::from(
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    );

    assert_eq!(solve1(&data), 136);

    assert_eq!(solve2(&data), 64);
}

pub fn solve1(data: &String) -> usize {
    // let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    // let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    println!("Part 2 = {}", res);
    res
}
