#[test]

fn test() {
    let example1 = String::from(
"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"
    );

    println!("Example 1:");
    assert_eq!(solve1(&example1), 54);
    // assert_eq!(solve2(&example1), 47);
}

use std::collections::HashMap;
use nalgebra::DMatrix;

pub fn solve1(data: &String) -> usize {
    let mut graph = HashMap::<&str, Vec<&str>>::new();
    for line in data.lines() {
        let (src, neighbors) = line.split_once(':').unwrap();
        for dst in neighbors.split_whitespace() {
            graph.entry(dst)
                .and_modify(|adj| {adj.push(src);})
                .or_insert(vec![src]);
            graph.entry(src)
                .and_modify(|adj| {adj.push(dst);})
                .or_insert(vec![dst]);
        }
    }

    let mut idx_map = HashMap::<&str, usize>::new();

    for (idx, key) in graph.keys().enumerate() {
        idx_map.insert(&key, idx);
    }

    let mut matrix = DMatrix::<f32>::identity(graph.len(), graph.len());
    for (key, val) in graph {
        matrix[(idx_map[key], idx_map[key])] = val.len() as f32;
        for neighbor in val {
            matrix[(idx_map[key], idx_map[neighbor])] = -1.0;
            matrix[(idx_map[neighbor], idx_map[key])] = -1.0;
        }
    }

    let eigen_decomp = matrix.symmetric_eigen();
    let eigen_vals = eigen_decomp.eigenvalues;
    let eigen_vecs = eigen_decomp.eigenvectors;

    let mut min_index = 0;
    let mut min_value = f32::MAX;
    let mut second_min_index = 0;
    let mut second_min_value = f32::MAX;
    for (i, &value) in eigen_vals.iter().enumerate() {
        if value < min_value {
            second_min_index = min_index;
            second_min_value = min_value;
            min_index = i;
            min_value = value;
        } else if value < second_min_value {
            second_min_index = i;
            second_min_value = value;
        }
    }

    let mut pos = 0;
    let mut neg = 0;
    eigen_vecs.column(second_min_index).iter().for_each(|&v| {
        if v >= 0.0 { pos += 1; }
        else { neg += 1; }
    });

    let res = pos * neg;
    println!("Part 1 = {}", res);
    res
}

// pub fn solve2(_data: &String) -> usize {
//     let res = 0;
//     println!("Part 2 = {}", res);
//     res
// } No part 2