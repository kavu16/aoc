#[test]

fn test() {
    let data = String::from(
"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    );

    solve1(&data);

    solve2(&data);
}

use std::{collections::HashMap, cmp::Ordering};

pub fn poker_value(card: char) -> Option<i32> {
    match card {
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'J' => Some(11),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None
    }
}

pub fn poker_value_wild(card: char) -> Option<i32> {
    match card {
        '2' => Some(2),
        '3' => Some(3),
        '4' => Some(4),
        '5' => Some(5),
        '6' => Some(6),
        '7' => Some(7),
        '8' => Some(8),
        '9' => Some(9),
        'T' => Some(10),
        'J' => Some(1),
        'Q' => Some(12),
        'K' => Some(13),
        'A' => Some(14),
        _ => None
    }
}

pub fn cmp_poker(hand1: &&str, hand2: &&str) -> Ordering {
    if let (Some(card1), Some(card2)) = (hand1.chars().next(), hand2.chars().next()) {
        if let (Some(value1), Some(value2)) = (poker_value(card1), poker_value(card2)) {
            match value1.cmp(&value2) {
                std::cmp::Ordering::Equal => {
                    cmp_poker(&&hand1[1..], &&hand2[1..])
                }
                other => other,
            }
        } else {
            std::cmp::Ordering::Equal
        }
    } else {
        std::cmp::Ordering::Equal
    }
}

pub fn cmp_poker_wild(hand1: &&str, hand2: &&str) -> Ordering {
    if let (Some(card1), Some(card2)) = (hand1.chars().next(), hand2.chars().next()) {
        if let (Some(value1), Some(value2)) = (poker_value_wild(card1), poker_value_wild(card2)) {
            match value1.cmp(&value2) {
                std::cmp::Ordering::Equal => {
                    cmp_poker_wild(&&hand1[1..], &&hand2[1..])
                }
                other => other,
            }
        } else {
            std::cmp::Ordering::Equal
        }
    } else {
        std::cmp::Ordering::Equal
    }
}

pub fn solve1(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut bids: Vec<(&str, i32)> = vec![];

    for line in data {
        let bid: Vec<&str> = line.split_whitespace().collect();
        bids.push((bid[0], bid[1].parse::<i32>().unwrap()));
    }

    bids.sort_by(|(a, _), (b, _)| {
        let mut unique_a = HashMap::new();
        let mut unique_b = HashMap::new();

        a.chars().for_each(|c| {
            let count = unique_a.entry(c).or_insert(0);
            *count += 1;
        });

        b.chars().for_each(|c| {
            let count = unique_b.entry(c).or_insert(0);
            *count += 1;
        });

        let mut a_counts: Vec<&i32> = unique_a.values().collect();
        let mut b_counts: Vec<&i32> = unique_b.values().collect();

        a_counts.sort_by_key(|n| -1 * **n);
        b_counts.sort_by_key(|n| -1 * **n);

        match a_counts.cmp(&b_counts) {
            std::cmp::Ordering::Equal => {
                cmp_poker(a, b)
            }
            other => other,
        }
    });

    let mut res = 0;
    for (index, (_hand, bid)) in bids.iter().enumerate() {
        // println!("Hand = {hand}, I bet {bid}");
        res += bid * (index as i32 + 1);
    }

    println!("Part 1 = {}", res);
}

pub fn solve2(data: &String) {
    let data = data.lines().collect::<Vec<_>>();
    let mut bids: Vec<(&str, i32)> = vec![];

    for line in data {
        let bid: Vec<&str> = line.split_whitespace().collect();
        bids.push((bid[0], bid[1].parse::<i32>().unwrap()));
    }

    bids.sort_by(|(a, _), (b, _)| {
        let mut unique_a = HashMap::new();
        let mut unique_b = HashMap::new();

        a.chars().for_each(|c| {
            let count = unique_a.entry(c).or_insert(0);
            *count += 1;
        });

        b.chars().for_each(|c| {
            let count = unique_b.entry(c).or_insert(0);
            *count += 1;
        });
        
        if let Some(wild_a) = unique_a.remove(&'J') {
            if unique_a.is_empty() {
                unique_a.insert('J', wild_a);
            } else {
                let (&a_max_k, _a_max_v) = unique_a.iter().max_by_key(|entry | entry.1).unwrap();
                unique_a.entry(a_max_k).and_modify(|n| *n += wild_a);
            }
        }

        if let Some(wild_b) = unique_b.remove(&'J') {
            if unique_b.is_empty() {
                unique_b.insert('J', wild_b);
            } else {
                let (&b_max_k, _a_max_v) = unique_b.iter().max_by_key(|entry | entry.1).unwrap();
                unique_b.entry(b_max_k).and_modify(|n| *n += wild_b);
            }
        }

        let mut a_counts: Vec<&i32> = unique_a.values().collect();
        let mut b_counts: Vec<&i32> = unique_b.values().collect();

        a_counts.sort_by_key(|n| -1 * **n);
        b_counts.sort_by_key(|n| -1 * **n);

        match a_counts.cmp(&b_counts) {
            std::cmp::Ordering::Equal => {
                cmp_poker_wild(a, b)
            }
            other => other,
        }
    });

    let mut res = 0;
    for (index, (_hand, bid)) in bids.iter().enumerate() {
        // println!("Hand = {hand}, I bet {bid}");
        res += bid * (index as i32 + 1);
    }
    println!("Part 2 = {}", res);
}