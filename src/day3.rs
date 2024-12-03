use regex::Regex;

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_part2 = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();
    let line = input.as_str();

    ////////////////////////////
    // part 1
    ////////////////////////////
    for caps in re.captures_iter(line) {
        let mult = &caps[1].parse::<i32>().unwrap() * &caps[2].parse::<i32>().unwrap();
        result += mult;
    }

    ////////////////////////////
    // part 2
    ////////////////////////////
    let mut disable = false;

    for caps in re_part2.captures_iter(line) {
        if &caps[0] == "do()" {
            disable = false;
        } else if &caps[0] == "don't()" {
            disable = true;
        }

        if !disable && *&caps[0].starts_with("mul"){
            let mult = &caps[1].parse::<i64>().unwrap() * &caps[2].parse::<i64>().unwrap();
            result_part2 += mult;
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}