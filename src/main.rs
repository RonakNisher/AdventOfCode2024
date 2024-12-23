use std::env;
use std::fs;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day22;
mod day23;

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
        "8" => day8::solve(contents),
        "9" => day9::solve(contents),
        "10" => day10::solve(contents),
        "11" => day11::solve(contents),
        "12" => day12::solve(contents),
        "13" => day13::solve(contents),
        "14" => day14::solve(contents),
        "15" => day15::solve(contents),
        "16" => day16::solve(contents),
        "17" => day17::solve(contents),
        "18" => day18::solve(contents),
        "19" => day19::solve(contents),
        "20" => day20::solve(contents),
        "22" => day22::solve(contents),
        "23" => day23::solve(contents),
        _ => println!("No solution for day {}", day),
    }
}