use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let day = &args[1];

    println!("Day: {}", day);

    let filepath = format!("src/inputs/day{}_input.txt", day);

    println!("Filepath: {}", filepath);
    let contents = fs::read_to_string(filepath).expect("Should have been able to read the file");

    match day.as_str() {
        "1" => day1::solve(contents),
        "2" => day2::solve(contents),
        "3" => day3::solve(contents),
        "4" => day4::solve(contents),
        "5" => day5::solve(contents),
        "6" => day6::solve(contents),
        "7" => day7::solve(contents),
        _ => println!("No solution for day {}", day),
    }
}