
use itertools::Itertools;

pub fn solve(input: String) {
    let mut result = 0;
    let mut keys: Vec<Vec<i32>> = Vec::new();
    let mut locks: Vec<Vec<i32>> = Vec::new();

    let input_lines = input.split("\n\n").collect_vec();

    input_lines.iter().for_each(|line| {
        let schematic = line.split("\n").map(|l| l.chars().collect_vec()).collect_vec();
        let is_lock = schematic[0].iter().all(|c| c == &'#');
        let mut heights: Vec<i32> = Vec::new();

        for col in 0..schematic[0].len() {
            let mut count = 0;
            for row in 1..schematic.len() - 1 {
                if schematic[row][col] == '#' {
                    count += 1;
                }
            }

            heights.push(count);
        }

        if is_lock {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    });

    for key in keys.iter() {
        for lock in locks.iter() {
            let mut is_valid = true;
            for i in 0..key.len() {
                if key[i] + lock[i] > 5 {
                    is_valid = false;
                    break;
                }
            }

            if is_valid {
                result += 1;
            }
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("*******************");
}