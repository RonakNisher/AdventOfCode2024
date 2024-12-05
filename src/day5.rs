use std::collections::HashMap;
use itertools::Itertools;

pub fn fix_order(page_number: &Vec<&str>, rules: &HashMap<String, Vec<String>>) -> i32 {
    let mut new_order: Vec<&str> = page_number.clone();

    for i in 0..new_order.len() {
        let page = new_order[i];
        for index in 0..i {
            let mut did_swap = false;
            let prev_page = new_order[index].to_string();
            if let Some(rule) = rules.get(page) {
                if rule.contains(&prev_page) {
                    new_order.swap(i, index);
                    did_swap = true;
                }
            }

            if did_swap && is_valid_order(&new_order, rules) {
                return new_order[new_order.len() / 2].parse::<i32>().unwrap();
            }
        }
    }

    panic!("Could not fix order");
}
   
pub fn is_valid_order(page_number: &Vec<&str>, rules: &HashMap<String, Vec<String>>) -> bool {
    for (i , page) in page_number.iter().enumerate() {
        // check if all elements to your left are not breaking the rules
        for index in 0..i {
            let prev_page = page_number[index].to_string();
            if let Some(rule) = rules.get(*page) {
                if rule.contains(&prev_page) {
                    return false;
                }
            }
        }
    }

    return true;
}
pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let (rule_lines, page_lines) = input.split_terminator("\n\n").collect_tuple().unwrap();

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();
    let mut page_numbers: Vec<Vec<&str>> = Vec::new();

    for line in rule_lines.lines() {
        let (left, right) = line.split('|').collect_tuple().unwrap();

        if rules.contains_key(left) {
            rules.get_mut(left).unwrap().push(right.to_string());
        } else {
            rules.insert(left.to_string(), vec![right.to_string()]);
        }
    }

    for line in page_lines.lines() {
        page_numbers.push(line.split(',').collect_vec());
    }

    ////////////////////////////
    // part 1
    ////////////////////////////
    
    for page_number in page_numbers {
        if is_valid_order(&page_number, &rules) {
            result += page_number[page_number.len() / 2].parse::<i32>().unwrap();
        }
        else {
            ////////////////////////////
            // part 2
            ////////////////////////////
            
            result_part2 += fix_order(&page_number, &rules);
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}