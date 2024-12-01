use itertools::Itertools;

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        // println!("{}", line);
        let ids = line.split_whitespace().collect_vec();

        left_list.push(ids[0].parse::<i32>().unwrap());
        right_list.push(ids[1].parse::<i32>().unwrap());

    });

    ////////////////////////////
    // part 1
    ////////////////////////////
    left_list.sort();
    right_list.sort();

    left_list.iter().zip(right_list.iter()).for_each(|(left, right)| {
        result += left.abs_diff(*right);
    });

    ////////////////////////////
    // part 2
    ////////////////////////////
    left_list.iter().for_each(|id: &i32| {
        let count = right_list.iter().filter(|&x| x == id).count() as i32;
        result_part2 += id * count;
    });

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}