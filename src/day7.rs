use itertools::Itertools;

pub fn get_valid_combinations(output: i64, input: &Vec<i64>, index: i64, current_cumulative_value: i64, is_part_2: bool) -> i64 {
    let mut result = 0;

    // println!("Output: {}, Input: {:?}, Index: {}, Current Cumulative Value: {}", output, input, index, current_cumulative_value);

    if index == input.len() as i64 {
        if current_cumulative_value == output {
            return 1;
        }

        return 0;
    }

    // check for valid combiantions with * or +
    result += get_valid_combinations(output, input, index + 1, current_cumulative_value + input[index as usize], is_part_2);
    result += get_valid_combinations(output, input, index + 1, current_cumulative_value * input[index as usize], is_part_2);


    if is_part_2 {
        let new_cumulative_value = (current_cumulative_value.to_string() + input[index as usize].to_string().as_str()).parse::<i64>().unwrap();
        result += get_valid_combinations(output, input, index + 1, new_cumulative_value, is_part_2);
    }

    return result;
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    input.lines().for_each(|line| {
        let (output_line, input_line) = line.split(":").collect_tuple().unwrap();
        let output = output_line.parse::<i64>().unwrap();
        let input = input_line.trim().split(" ").map(|x| {x.parse::<i64>().unwrap()}).collect_vec();

        if get_valid_combinations(output, &input, 1, input[0], false /*is_part_2*/) != 0 {
            result += output;
        }
        else {
            // part 2
            if get_valid_combinations(output, &input, 1, input[0], true /*is_part_2*/) != 0 {
                result_part2 += output;
            }
        }
    });

    result_part2 += result; // part 2 results are cumulative of part 1

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}