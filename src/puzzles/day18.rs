#[test]

fn test() {
    let data = String::from(
"R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
    );

    assert_eq!(solve1(&data), 62);

    assert_eq!(solve2(&data), 0);
}

pub fn solve1(data: &String) -> usize {
    let _data = data.lines().collect::<Vec<_>>();

    let res = 0;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> usize {
    let _data = data.lines().collect::<Vec<_>>();
    
    let res = 0;
    println!("Part 2 = {}", res);
    res
}
