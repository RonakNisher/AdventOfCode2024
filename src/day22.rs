
use std::{collections::{HashMap, HashSet}, ops::BitXor};

pub fn get_next_secret_number(current_secret: u64) -> u64 {
    // let mut count = 0;
    let mut result = current_secret * 64;
    // mix
    result = current_secret.bitxor(result);
    // prune
    result = result % 16777216;

    // step 2
    let step2: u64 = result / 32;
    result = result.bitxor(step2);
    result = result % 16777216;

    // step 3
    let step3 = result * 2048;
    result = result.bitxor(step3);
    result = result % 16777216;

    return result;
}


pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let mut sequence_bananas_map: HashMap<(i32, i32, i32, i32), i32> = HashMap::new();

    for line in input.lines() {
        let mut secret_number = line.parse::<u64>().unwrap();
        let mut prev_units_digit = (secret_number % 10) as i32;
        let mut current_units_digit: i32;
        let mut prev_changes: Vec<i32> = Vec::new();
        let mut bananas: Vec<i32> = Vec::new();

        
        for _i in 0..2000 {
            let next_secret = get_next_secret_number(secret_number);

            current_units_digit = (next_secret % 10) as i32;
            let change = current_units_digit - prev_units_digit;
            prev_changes.push(change);
            bananas.push(current_units_digit);

            prev_units_digit = current_units_digit;
            secret_number = next_secret;
        }

        let mut seen_sequences: HashSet<(i32, i32, i32, i32)> = HashSet::new();
        for i in 3..prev_changes.len() {
            let sequence = (prev_changes[i-3], prev_changes[i-2], prev_changes[i-1], prev_changes[i]);
            let banana = bananas[i];

            if seen_sequences.contains(&sequence) {
                continue;
            }

            seen_sequences.insert(sequence);
            *sequence_bananas_map.entry(sequence).or_insert(0) += banana;
        }

        result += secret_number;
    }
    
    let max = sequence_bananas_map.iter().max_by_key(|x| x.1).unwrap();
    println!("Max: {:?}", max);
    result_part2 = *max.1 as u64;
    

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}