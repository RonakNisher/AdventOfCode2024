use std::collections::HashMap;

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let bricks: Vec<u64> = input.split(" ").map(|x| x.parse().unwrap()).collect();
    let mut bricks_map: HashMap<u64, u64> = HashMap::new();
    bricks.iter().for_each(|&x| { *bricks_map.entry(x).or_insert(0) += 1; });

    let num_blinks_part1 = 25;
    let num_blinks_part2 = 75;
    for i in 0..num_blinks_part2 {
        let mut new_bricks_map: HashMap<u64, u64> = HashMap::new();

        for (brick, count) in bricks_map.iter() {
            if brick == &0 {
                *new_bricks_map.entry(1).or_insert(0) += count;
                continue;
            }

            let brick_str = brick.to_string();

            if brick_str.len() % 2 == 0 {
                let half = brick_str.len() / 2;
                let (left, right) = brick_str.split_at(half);
                let left_digit = left.parse::<u64>().unwrap();
                let right_digit = right.parse::<u64>().unwrap();

                *new_bricks_map.entry(left_digit).or_insert(0) += count;
                *new_bricks_map.entry(right_digit).or_insert(0) += count;
                continue;
            }

            *new_bricks_map.entry(brick * 2024).or_insert(0) += count;
        }

        bricks_map = new_bricks_map;

        if i == num_blinks_part1 - 1 {
            result = bricks_map.values().sum();
        }

        if i == num_blinks_part2 - 1 {
            result_part2 = bricks_map.values().sum();
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}