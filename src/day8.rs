use std::collections::{HashMap, HashSet};
use itertools::Itertools;

pub fn is_valid_location(node: &(i32, i32), max_row: i32, max_col: i32) -> bool {
    let row = node.0;
    let col = node.1;

    if row < 0 || row >= max_row {
        return false;
    }

    if col < 0 || col >= max_col {
        return false;
    }

    return true;
}

pub fn get_antinodes_count(antennas_map: &HashMap<char, Vec<(i32, i32)>>, max_row: i32, max_col: i32, min_dist: i32, max_dist: i32) -> i32 {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for antenna in antennas_map.keys() {
        let nodes = antennas_map.get(antenna).unwrap();
        for node_pair in nodes.iter().combinations(2) {
            let node1 = node_pair[0];
            let node2 = node_pair[1];

            
            for dist in min_dist..=max_dist {
                let mut added_some_antinodes = false;
                let antinode1 = (node1.0 + dist * (node1.0 - node2.0), node1.1 + dist * (node1.1 - node2.1));
                let antinode2 = (node2.0 + dist * (node2.0 - node1.0), node2.1 + dist * (node2.1 - node1.1));
    
                if is_valid_location(&antinode1, max_row, max_col) {
                    antinodes.insert(antinode1);
                    added_some_antinodes = true;
                }
    
                if is_valid_location(&antinode2, max_row, max_col) {
                    antinodes.insert(antinode2);
                    added_some_antinodes = true;
                }

                if !added_some_antinodes {
                    break;
                }
            }
        }
    }

    return antinodes.len() as i32;
}

pub fn solve(input: String) {
    let result;
    let result_part2;

    let mut antennas_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let max_row = input.lines().count() as i32;
    let mut max_col = 0 as i32;

    for (r, line) in input.lines().enumerate() {
        for (c, char) in line.chars().enumerate() {
            if char != '.' {
                antennas_map.entry(char).or_insert(Vec::new()).push((r as i32, c as i32));
            }
        }

        if max_col == 0 {
            max_col = line.chars().count() as i32;
        }
    };

    result = get_antinodes_count(&antennas_map, max_row, max_col, 1, 1);
    result_part2 = get_antinodes_count(&antennas_map, max_row, max_col, 0, max_row.max(max_col));

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}