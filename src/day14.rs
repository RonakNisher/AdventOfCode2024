
use std::vec;

use itertools::Itertools;

struct Robot {
    x: i64,
    y: i64,
    vx: i64,
    vy: i64

}

impl Robot {
    fn new(x: i64, y: i64, vx: i64, vy: i64) -> Robot {
        Robot { x, y, vx, vy }
    }

    fn _print(&self) {
        println!("x={}, y={}, vx={}, vy={}", self.x, self.y, self.vx, self.vy);
    }
}

fn print_robots_on_grid(robots: &Vec<Robot>, width: i64, height: i64) {
    let mut grid = vec![vec!['.'; width as usize]; height as usize];

    robots.iter().for_each(|robot| {
        grid[robot.y as usize][robot.x as usize] = '#';
    });

    for y in 0..height {
        for x in 0..width {
            print!("{}", grid[y as usize][x as usize]);
        }
        println!();
    }
}

fn find_result_part1(robots: &Vec<Robot>, width: i64, height: i64) -> i64 {
    let mid_x = width / 2;
    let mid_y = height / 2;

    let mut quadrants = vec![0,0,0,0];

    robots.iter().for_each(|robot| {
        if robot.x < mid_x && robot.y < mid_y {
            quadrants[0] += 1;
        }
        else if robot.x > mid_x && robot.y < mid_y {
            quadrants[1] += 1;
        }
        else if robot.x < mid_x && robot.y > mid_y {
            quadrants[2] += 1;
        }
        else if robot.x > mid_x && robot.y > mid_y {
            quadrants[3] += 1;

        }
    });

    return quadrants.iter().fold(1, |acc, x| acc * x);
}

// constructor
pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let width = 101;
    let height = 103;
    let max_seconds = 10000;

    let mut robots:Vec<Robot> = Vec::new();
    let re = regex::Regex::new(r"p=(-?\d*),(-?\d*) v=(-?\d*),(-?\d*)").unwrap();

    input.lines().for_each(|line| {
        let matches = re.captures(line).unwrap();
        let px = matches.get(1).unwrap().as_str().parse::<i64>().unwrap();
        let py = matches.get(2).unwrap().as_str().parse::<i64>().unwrap();
        let vx = matches.get(3).unwrap().as_str().parse::<i64>().unwrap();
        let vy = matches.get(4).unwrap().as_str().parse::<i64>().unwrap();

        let robot = Robot::new(px, py, vx, vy);
        robots.push(robot);
    });

    for i in 0..max_seconds {
        robots.iter_mut().for_each(|robot| {
            let mut new_x = robot.x + robot.vx;
            if new_x < 0 {
                new_x = width + new_x;
            }
            else if new_x >= width {
                new_x = new_x % width;
            }

            let mut new_y = robot.y + robot.vy;
            if new_y < 0 {
                new_y = height + new_y;
            }
            else if new_y >= height {
                new_y = new_y % height;
            }

            robot.x = new_x;
            robot.y = new_y;
        });

        let sorted_robots = robots.iter().map(|robot| (&robot.x, robot.y)).sorted_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0))).collect_vec();
        let mut count = 0;

        sorted_robots.iter().tuple_windows().for_each(|(a, b)| {
            let mut diff = 10;
            if a.1 == b.1 
            {
                diff = (b.0 - a.0).abs();
            }
            
            if diff == 1 {
                count += 1;

                if count > 10 {
                    result_part2 = i + 1;
                    return;
                }
            }
            else if diff > 1
            {
                count = 0;
            }
        });
        
        if i == 99 {
            result = find_result_part1(&robots, width, height);
        }
        
        if result_part2 != 0 {
            print_robots_on_grid(&robots, width, height);
            break;
        }
    }
    
    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}