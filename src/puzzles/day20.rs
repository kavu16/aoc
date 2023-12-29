use std::collections::HashMap;

#[test]

fn test() {
    let example1 = String::from(
"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"
    );

    let example2 = String::from(
"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"
    );

    assert_eq!(solve1(&example1), 32_000_000);
    assert_eq!(solve1(&example2), 11_687_500);

    assert_eq!(solve2(&example1), 0);
    assert_eq!(solve2(&example2), 0);
}



pub fn solve1(data: &String) -> u32 {
    let res = 0;
    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> u128 {
    let res = 0;
    println!("Part 2 = {}", res);
    res
}