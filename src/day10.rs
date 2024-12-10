use std::collections::HashSet;

pub fn get_total_trailheads(grid: &Vec<Vec<char>>, visited: &mut HashSet<(i32, i32)>, prev_height: char, current_row: i32, current_col: i32) -> i32 {
    let mut trailheads = 0;

    if current_row < 0 || current_row >= grid.len() as i32 || current_col < 0 || current_col >= grid[0].len() as i32 {
        return 0;
    }

    // for test inputs
    if grid[current_row as usize][current_col as usize] == '.' {
        return 0;
    }

    let current_digit = grid[current_row as usize][current_col as usize].to_digit(10).unwrap() as i32;
    let prev_height_digit = prev_height.to_digit(10).unwrap() as i32;

    if prev_height_digit >= current_digit || (current_digit - prev_height_digit).abs() > 1 {
        return 0;
    }

    if current_digit == 9 {
        visited.insert((current_row, current_col));

        return 1;
    }

    let current_height = grid[current_row as usize][current_col as usize];

    // top
    trailheads += get_total_trailheads(grid, visited, current_height, current_row - 1, current_col);

    // right
    trailheads += get_total_trailheads(grid, visited, current_height, current_row, current_col + 1);

    // bottom
    trailheads += get_total_trailheads(grid, visited, current_height, current_row + 1, current_col);

    // left
    trailheads += get_total_trailheads(grid, visited, current_height, current_row, current_col - 1);
    
    return trailheads;
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    for (r, row) in grid.iter().enumerate() {
        for (c, _) in row.iter().enumerate() {
            if grid[r][c] == '0' {
                let mut visited = HashSet::new();
                let top = get_total_trailheads(&grid, &mut visited, '0', r as i32 - 1, c as i32);
                let right = get_total_trailheads(&grid, &mut visited, '0', r as i32, c as i32 + 1);
                let bottom = get_total_trailheads(&grid, &mut visited, '0', r as i32 + 1, c as i32);
                let left = get_total_trailheads(&grid, &mut visited, '0', r as i32, c as i32 - 1);

                let trailheads = top + right + bottom + left;

                result += visited.len() as i32;
                result_part2 += trailheads;
            }
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}