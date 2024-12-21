
use std::collections::HashMap;
use itertools::Itertools;

pub fn check_design_possible(design: &str, patterns: &Vec<&str>, index: usize, seen_designs: &mut HashMap<String, u64>) -> u64 {
    if index >= design.len() {
        return 1;
    }
    
    let mut count = 0;

    if let Some(substring) = design.get(index..) {
        if let Some(&seen) = seen_designs.get(substring) {
            return seen;
        }

        for pattern in patterns {
            if substring.starts_with(pattern) {
                count += check_design_possible(design, patterns, index + pattern.len(), seen_designs);
            }
        }
    }
    
    seen_designs.insert(design[index..].to_string(), count);

    return count;
}


pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let (patterns_line, designs_line) = input.split("\n\n").collect_tuple().unwrap();
    let patterns = patterns_line.split(",").map(|x| x.trim()).collect::<Vec<&str>>();
    let designs = designs_line.split("\n").map(|x| x.trim()).collect::<Vec<&str>>();

    for design in designs {
        let mut seen_designs_map: HashMap<String, u64> = HashMap::new();
        let count = check_design_possible(design, &patterns, 0, &mut seen_designs_map);
        if count > 0 {
            result += 1;
        }

        result_part2 += count;
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}