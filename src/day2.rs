use itertools::Itertools;

pub fn is_valid(previous: i32, current: i32, direction: i32) -> bool {
     // check direction valid
     if direction == -1 {
        // decreasing
        if current > previous {
            return  false;
        }
    }
    else {
        // increasing
        if current < previous {
            return false;
        }
    }

    // check distance valid
    let dist = (current - previous).abs();

    if dist > 3 || dist < 1 {
        return false;
    }

    return true;
}

pub fn is_valid_result(level: &Vec<i32>) -> bool {
    let mut prev = level[0];
    let direction = if level[0] < level[1] {1} else {-1};
    let mut i = 1;

    while i < level.len() {
        let current = level[i];
        if !is_valid(prev, current, direction) {
            return false;
        }
        i += 1;
        prev = current;
    }

    return true;
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    input.lines().for_each(|line| {
        // println!("{}", line);
        let level = line.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect_vec();

        if !is_valid_result(&level) {
            for i in 0..level.len() {
                let mut new_level = level.clone();
                new_level.remove(i);
                if is_valid_result(&new_level) {
                    result_part2 += 1;
                    break;
                }
            }
        }
        else {
            result += 1;
            result_part2 += 1;
        }
    });

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}