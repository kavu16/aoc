use std::collections::HashMap;

#[test]

fn test() {
    let data = String::from(
"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    );

    assert_eq!(solve1(&data), 1320);

    assert_eq!(solve2(&data), 145);
}

pub fn solve1(data: &String) -> u32 {
    // let data = data.lines().collect::<Vec<_>>();
    let res = data.split(',')
                        .map(|s| s.as_bytes())
                        .map(|b| b.iter().fold(0u32, |a, &b| {
                            ((a + b as u32) * 17) % 256
                        }))
                        .sum();

    println!("Part 1 = {}", res);
    res
}

pub fn solve2(data: &String) -> u32 {
    // let data = data.lines().collect::<Vec<_>>();
    let mut hashmap: HashMap<u32, Vec<(&str, u32)>> = HashMap::new();
    
    data.split(',').for_each(|hash| {
        if hash.contains("=") {
            let mut key = 0;
            if let Some((key_str, val)) = hash.split_once("=") {
                for &b in key_str.as_bytes() {
                    key = ((key + b as u32) * 17) % 256;
                }
                let val = val.parse::<u32>().ok().unwrap();
    
                if let Some(lens_box) = hashmap.get_mut(&key) {
                    let mut i = 0;
                    let mut replaced = false;
                    while i < lens_box.len() {
                        if lens_box[i].0 == key_str {
                            lens_box[i].1 = val;
                            replaced = true;
                            break;
                        }
                        i += 1;
                    }
                    if !replaced {
                        lens_box.push((key_str, val));
                    }
                } else {
                    hashmap.insert(key, vec![(key_str, val)]);
                }
            } else {
                ()
            }
            
        } else {
            let mut key = 0;
            if let Some((key_str, _)) = hash.split_once("-") {
                for &b in key_str.as_bytes() {
                    key = ((key + b as u32) * 17) % 256;
                }
                if let Some(lens_box) = hashmap.get_mut(&key) {
                    let mut i = 0;
                    while i < lens_box.len() {
                        if lens_box[i].0 == key_str {
                            lens_box.remove(i);
                            break;
                        }
                        i += 1;
                    }
                }
            } else {
                ()
            }
        }
    });

    let mut res = 0;
    for (box_num, lens_box) in hashmap {
        for (i, (_key_str, focal_length)) in lens_box.iter().enumerate() {
            // println!("Label {}: Box Num {} * slot num {} * focal length {}", key_str, box_num + 1, i+1, focal_length);
            res += (box_num + 1)*((i+1) as u32)*focal_length;
        }
    }

    println!("Part 2 = {}", res);
    res
}
