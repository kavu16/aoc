#[test]
fn test() {
    solve1(&String::from(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    ));
    solve2(&String::from(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    ));
    solve_aho_corasick(&String::from(
        "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet"
    ));
}

pub fn solve1(data: &String) {
    let mut sum = 0;
    for line in data.lines() {
        let nums = line.chars()
                                .filter(|l| l.is_numeric())
                                .map(|d| d.to_digit(10).unwrap())
                                .collect::<Vec<u32>>();
        let first = *nums.iter().nth(0).unwrap();
        let last = *nums.iter().last().unwrap();

        sum += (first as i32) * 10 + last as i32;
    }
    println!("Sum1 = {}", sum);
}

pub fn solve2(data: &String) {
    let mut sum = 0;
    let nums = ["one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];

    for line in data.split('\n') {
        match line {
            "" => break,
            _ => {
                let mut first = None;
                'out: for i in 0..line.len() {
                    for (index, num) in nums.iter().enumerate() {
                        if i + num.len() > line.len() {
                            continue;
                        }
                        if line[i..i+num.len()] == **num {
                            first = Some(1 + index / 2);
                            break 'out;
                        }
                    }
                }

                let Some(first) = first else {panic!("invalid input"); };

                let mut last = None;
                'out: for i in (0..line.len()).rev() {
                    for (index, num) in nums.iter().enumerate() {
                        if i + num.len() > line.len() {
                            continue;
                        }
                        if line[i..i+num.len()] == **num {
                            last = Some(1 + index / 2);
                            break 'out;
                        }
                    }
                }

                let Some(last) = last else {panic!("invalid input"); };

                sum += 10 * first as i32 + last as i32;
            }
        }
    }
    println!("Sum2 = {}", sum);
}

use::aho_corasick::AhoCorasick;

pub fn solve_aho_corasick(data: &String) {
    let mut sum = 0;
    let nums = &["one", "1", "two", "2", "three", "3", "four", "4", "five", "5", "six", "6", "seven", "7", "eight", "8", "nine", "9"];

    let ac = AhoCorasick::new(nums).unwrap();
    for line in data.lines() {
        let matches = ac.find_overlapping_iter(line).collect::<Vec<_>>();

        let first = matches.iter().nth(0).unwrap().pattern().as_u32()/2 + 1;
        let last = matches.iter().last().unwrap().pattern().as_u32()/2 + 1;

        sum += 10 * first as i32 + last as i32;
    }
    println!("SumAC = {}", sum);
}