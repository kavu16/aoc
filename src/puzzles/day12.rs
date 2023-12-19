use std::collections::HashMap;

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

    assert_eq!(solve1(&data), 21);

    assert_eq!(solve2(&data), 525_152);
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Spring {
    Damaged,
    Working,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct ParseConditionRecordError;

impl TryFrom<char> for Spring {
    type Error = ParseConditionRecordError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Spring::Working),
            '#' => Ok(Spring::Damaged),
            '?' => Ok(Spring::Unknown),
            _ => Err(ParseConditionRecordError),
        }
    }
}

struct ConditionRecord {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

// Backtracking solution good enough for small strings, but too slow
// unable to implement cache key with combo of strings and targets, building too complicated

// fn backtrack(curr: &str, target: &str, nums: &Vec<i32>) -> i32 {
//     if curr.len() == target.len() {
//         let curr = curr.split(".")
//                                  .map(|s| s.len() as i32)
//                                  .filter(|&n| n > 0)
//                                  .collect::<Vec<i32>>();
//         if curr == *nums {
//             return 1;
//         } else {
//             return 0;
//         }
//     }

//     let mut res = 0;
//     if target.chars().nth(curr.len()).unwrap() == '?' {
//         res += backtrack(&format!("{}{}",curr,"#")[..], target, nums);
//         res += backtrack(&format!("{}{}",curr,".")[..], target, nums);
//     } else {
//         let next = target.chars().nth(curr.len()).unwrap();
//         res += backtrack(&format!("{}{}",curr,next)[..], target, nums);
//     }

//     res
// }

// Credit github: andypymont

impl ConditionRecord {
    fn find_combos(
        &self,
        cache: &mut HashMap<(usize, usize), u64>, 
        spring_idx: usize, 
        group_idx: usize
    ) -> u64 {
        if let Some(arrangements) = cache.get(&(spring_idx, group_idx)) {
            return *arrangements;
        }
    
        let found_group = self.groups.get(group_idx).map_or(0, |group_size| {
            // Group out of bounds || contains operational springs || next spring is damaged
            if (spring_idx + group_size) > self.springs.len() || 
                    (0..*group_size).any(|i| self.springs.get(spring_idx + i) == Some(&Spring::Working)) || 
                    self.springs.get(spring_idx + group_size) == Some(&Spring::Damaged) {
                return 0;
            }

            // Group is valid
            self.find_combos(cache, spring_idx + group_size + 1, group_idx + 1)
        });

        let skip = match self.springs.get(spring_idx) {
            None => u64::from(group_idx >= self.groups.len()), // Checked all springs, 1 if matched all groups, 0 else remaining
            Some(&Spring::Damaged) => 0, // current spring is damaged, can't skip
            Some(_) => self.find_combos(cache, spring_idx + 1, group_idx), // This is key to the combos, skipping working or unknowns
        };

        let total_combos = found_group + skip;
        cache.insert((spring_idx, group_idx), total_combos);
        total_combos
    }
}

pub fn solve1(data: &String) -> u64 {
    let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;
    for line in data {
        let (syms, nums) = line.split_once(' ').unwrap();

        let nums: Vec<usize> = nums.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
        let syms: Vec<Spring> = syms.chars().map(|s| s.try_into().unwrap()).collect();

        let springs: ConditionRecord = ConditionRecord { springs: syms, groups: nums };
        let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

        res += springs.find_combos(&mut cache, 0, 0);
    }

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> u64 {
    let data = data.lines().collect::<Vec<_>>();
    let mut res = 0;

    for line in data {
        let (syms, nums) = line.split_once(' ').unwrap();
        // Unfold
        let syms = format!("{syms}?{syms}?{syms}?{syms}?{syms}");
        let nums = format!("{nums},{nums},{nums},{nums},{nums}");

        let nums: Vec<usize> = nums.split(',').map(|n| n.parse::<usize>().unwrap()).collect();
        let syms = syms.chars().map(|s| s.try_into().unwrap()).collect::<Vec<Spring>>();

        let springs: ConditionRecord = ConditionRecord { springs: syms, groups: nums };
        let mut cache: HashMap<(usize, usize), u64> = HashMap::new();

        res += springs.find_combos(&mut cache, 0, 0);
    }
    println!("Part 2 = {}", res);
    res
}