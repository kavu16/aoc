#[test]

fn test() {
    let data = String::from(
"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    );

    solve1(&data);

    solve2(&data);
}

pub fn solve1(data: &String) {
    let _data = data.lines().collect::<Vec<_>>();

    println!("Part 1 = {}", 0);
}

pub fn solve2(data: &String) {
    let _data = data.lines().collect::<Vec<_>>();

    println!("Part 2 = {}", 0);
}