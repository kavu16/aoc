#[test]

fn test() {
    let data = String::from(
"Time:      7  15   30
Distance:  9  40  200"
    );

    solve1(&data);

    solve2(&data);
}

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let bids: Vec<(&str, i32)> = vec![];

    for bid in data {
        bid = bid.split_whitespace().map(|(h, b)| (h, b.parse::<i32>().unwrap()));
    }

    println!("Part 1 = {}", res);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();

    println!("Part 2 = {}", res);
}