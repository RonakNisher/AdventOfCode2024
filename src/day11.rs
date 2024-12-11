use std::collections::HashMap;

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let bricks: Vec<u64> = input.split(" ").map(|x| x.parse().unwrap()).collect();
    let mut bricks_map: HashMap<u64, u64> = bricks.iter().map(|x| (*x, 1)).collect();

    let num_blinks_part1 = 25;
    let num_blinks_part2 = 75;
    for i in 0..num_blinks_part2 {
        let mut new_bricks: Vec<u64> = Vec::new();
        let mut new_bricks_map: HashMap<u64, u64> = HashMap::new();
        new_bricks.reserve(bricks.len() * 2);

        for (brick, count) in bricks_map.iter() {
            if brick == &0 {
                let new_val = new_bricks_map.get(&1).unwrap_or(&0) + 1 * count;
                new_bricks_map.insert(1, new_val);
                continue;
            }

            let brick_str = brick.to_string();

            if brick_str.len() % 2 == 0 {
                let half = brick_str.len() / 2;
                let (left, right) = brick_str.split_at(half);
                let left_digit = left.parse::<u64>().unwrap();
                let right_digit = right.parse::<u64>().unwrap();

                new_bricks_map.insert(left_digit, new_bricks_map.get(&left_digit).unwrap_or(&0) + 1 * count);
                new_bricks_map.insert(right_digit, new_bricks_map.get(&right_digit).unwrap_or(&0) + 1 * count);
                continue;
            }

            new_bricks_map.insert(brick * 2024, new_bricks_map.get(&(brick * 2024)).unwrap_or(&0) + 1 * count);
        }

        bricks_map = new_bricks_map.clone();

        if i == num_blinks_part1 - 1 {
            result = new_bricks_map.values().sum();
        }

        if i == num_blinks_part2 - 1 {
            result_part2 = new_bricks_map.values().sum();
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}