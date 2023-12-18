#[test]

fn test() {
    let data = String::from(
"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    );

    solve1(&data);

    solve2(&data);
}

fn backtrack(curr: &str, target: &str, nums: &Vec<i32>) -> i32 {
    if curr.len() == target.len() {
        let curr = curr.split(".")
                                 .map(|s| s.len() as i32)
                                 .filter(|&n| n > 0)
                                 .collect::<Vec<i32>>();
        if curr == *nums {
            return 1;
        } else {
            return 0;
        }
    }

    let mut res = 0;
    if target.chars().nth(curr.len()).unwrap() == '?' {
        res += backtrack(&format!("{}{}",curr,"#")[..], target, nums);
        res += backtrack(&format!("{}{}",curr,".")[..], target, nums);
    } else {
        let next = target.chars().nth(curr.len()).unwrap().to_string();
        res += backtrack(&format!("{}{}",curr,next)[..], target, nums);
    }

    return res;
}

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;
    for line in data {
        let (syms, nums) = line.split_once(' ').unwrap();
        let nums: Vec<i32> = nums.split(',').map(|n| n.parse::<i32>().unwrap()).collect();
        
        res += backtrack("", &syms, &nums);
    }

    println!("Part 1 = {}", res);
}

pub fn solve2(data: &String) {
    let _data = data.lines().collect::<Vec<_>>();

    println!("Part 2 = {}", 0);
}