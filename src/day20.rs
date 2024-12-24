

pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn advance_pos(pos: (usize, usize), dir: &Dir) -> (usize, usize) {
    match dir {
     Dir::Up => (pos.0 - 1, pos.1),
     Dir::Down => (pos.0 + 1, pos.1),
     Dir::Left => (pos.0, pos.1 - 1),
     Dir::Right => (pos.0, pos.1 + 1),
    }
}

pub fn find_cost(path: &Vec<(usize, usize)>, start: (usize, usize), end: (usize, usize)) -> i32 {
    let start_index = path.iter().position(|&pos| pos == start).unwrap();
    let end_index = path.iter().position(|&pos| pos == end).unwrap();

    return (end_index - start_index) as i32;
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let maze: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    maze.iter().enumerate().for_each(|(row, row_vec)| {
        row_vec.iter().enumerate().for_each(|(col, col_char)| {
            if *col_char == 'S' {
                start = (row as usize, col as usize);
            }
            else if *col_char == 'E' {
                end = (row as usize, col as usize);
            }
        });
    });

    let mut path: Vec<(usize, usize)> = Vec::new();
    let mut prev: (usize, usize) = start;
    path.push(prev);

    while prev != end {
        for direction in vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right] {
            let new_position = advance_pos(prev, &direction);

            let entry = maze[new_position.0][new_position.1];
            if entry == '#' || new_position == prev || entry == 'S' {
                continue;
            }

            if (entry == '.' || entry == 'E') && !path.contains(&new_position) {
                path.push(new_position);
                prev = new_position;
                break;
            }
        }
    }

    // now that we have the path, see if we can cut across the path with the given max cheat count
    let max_cheat_count_part1 = 2;
    let max_cheat_count_part2 = 20;
    
    for i in 0..path.len() {
        for j in i+1..path.len() {
            let cheat_start_pos = path[i];
            let cheat_end_pos = path[j];

            let manhattan_distance = (cheat_start_pos.0 as i32 - cheat_end_pos.0 as i32).abs() + (cheat_start_pos.1 as i32 - cheat_end_pos.1 as i32).abs();

            if manhattan_distance > max_cheat_count_part2 {
                continue;
            }

            let current_cost = find_cost(&path, cheat_start_pos, cheat_end_pos);
            let saved_cost = current_cost - manhattan_distance;

            if saved_cost >= 100 {
                result_part2 += 1;

                if manhattan_distance <= max_cheat_count_part1 {
                    result += 1;
                }
            }
        }
    }
    
    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}