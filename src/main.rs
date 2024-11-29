use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let day = &args[1];

    println!("Day: {}", day);

    let filepath = format!("src/inputs/day{}_input.txt", day);

    println!("Filepath: {}", filepath);
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    match day.as_str() {
        _ => println!("No solution for day {}", day),
    }
}