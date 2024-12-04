use itertools::Itertools;

pub fn _print_grid(grid: &Vec<Vec<char>>) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}

pub fn get_next_char(ch: char) -> char {
    return match ch {
        'X' => 'M',
        'M' => 'A',
        'A' => 'S',
        _ => panic!("Invalid character"),
    }
}

pub fn is_out_of_bounds(row: i32, col: i32, grid: &Vec<Vec<char>>) -> bool {
    if row < 0 || row >= grid.len() as i32 || col < 0 || col >= grid[0].len() as i32{
        return true;
    }

    return false;
}

pub fn check_top(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_top(grid, row - 1, col, get_next_char(verify_current_char));
}

pub fn check_bottom(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_bottom(grid, row + 1, col, get_next_char(verify_current_char));
}

pub fn check_left(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_left(grid, row, col - 1, get_next_char(verify_current_char));
}

pub fn check_right(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_right(grid, row, col + 1, get_next_char(verify_current_char));
}

pub fn check_top_left(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_top_left(grid, row - 1, col - 1, get_next_char(verify_current_char));
}

pub fn check_top_right(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_top_right(grid, row - 1, col + 1, get_next_char(verify_current_char));
}

pub fn check_bottom_left(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_bottom_left(grid, row + 1, col - 1, get_next_char(verify_current_char));
}

pub fn check_bottom_right(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    if is_out_of_bounds(row, col, grid){
        return 0;
    }

    if grid[row as usize][col as usize] != verify_current_char {
        return 0;
    }

    if verify_current_char == 'S' {
        return 1;
    }
    
    return check_bottom_right(grid, row + 1, col + 1, get_next_char(verify_current_char));
}

pub fn find_word(grid: &Vec<Vec<char>>, row: i32, col: i32, verify_current_char: char) -> i32 {
    return check_top_left(grid, row, col, verify_current_char) + 
        check_top_right(grid, row, col, verify_current_char) + 
        check_bottom_left(grid, row, col, verify_current_char) + 
        check_bottom_right(grid, row, col, verify_current_char) + 
        check_top(grid, row, col, verify_current_char) +
        check_bottom(grid, row, col, verify_current_char) + 
        check_left(grid, row, col, verify_current_char) + 
        check_right(grid, row, col, verify_current_char)
}

pub fn find_word_part2(grid: &Vec<Vec<char>>, row: i32, col: i32) -> i32 {

    let verify_current_char = 'M';

    // left MAS
    // M.S
    // .A.
    // M.S

    let left_found = check_bottom_right(grid, row - 1, col - 1, verify_current_char) == 1 && 
        check_top_right(grid, row + 1, col - 1, verify_current_char) == 1;

    if left_found {
        return  1;
    }

    // right MAS
    // S.M
    // .A.
    // S.M

    let right_found = check_bottom_left(grid, row - 1, col + 1, verify_current_char) == 1 && 
        check_top_left(grid, row + 1, col + 1, verify_current_char) == 1;

    if right_found {
        return 1
    }

    // top MAS
    // M.M
    // .A.
    // S.S
    let top_found = check_bottom_right(grid, row - 1, col - 1, verify_current_char) == 1 && 
        check_bottom_left(grid, row - 1, col + 1, verify_current_char) == 1;

    if top_found {
        return 1;
    }

    // bottom MAS
    // S.S
    // .A.
    // M.M
    let bottom_found = check_top_right(grid, row + 1, col - 1, verify_current_char) == 1 && 
        check_top_left(grid, row + 1, col + 1, verify_current_char) == 1;

    if bottom_found {
        return 1;
    }

    return 0
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let mut grid: Vec<Vec<char>> = Vec::new();

    input.lines().for_each(|line| {
        // println!("{}", line);
        let row = line.chars().collect_vec();
        grid.push(row);
    });

    // print_grid(&grid);

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            ////////////////////////////
            // part 1
            ////////////////////////////
            if grid[row][col] == 'X' {
                result += find_word(&grid, row as i32, col as i32, 'X');
            }

            ////////////////////////////
            // part 2
            ////////////////////////////
            if grid[row][col] == 'A' && row > 0 && row <= grid.len() - 2 && col > 0 && col <= grid.len() - 1 {
                result_part2 += find_word_part2(&grid, row as i32, col as i32);
            }
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}