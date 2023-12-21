#[test]

fn test() {
    let data = String::from(
""
    );

    assert_eq!(solve1(&data), 405);

    assert_eq!(solve2(&data), 400);
}

pub fn solve1(data: &String) -> u64 {
    let _data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> u64 {
    let _data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    println!("Part 2 = {}", res);
    res
}