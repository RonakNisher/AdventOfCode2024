
use std::collections::HashMap;
use itertools::Itertools;

pub fn get_next_pos(pos: (usize, usize), dir: char) -> (usize, usize) {
    match dir {
        '^' => (pos.0 - 1, pos.1),
        'v' => (pos.0 + 1, pos.1),
        '<' => (pos.0, pos.1 - 1),
        '>' => (pos.0, pos.1 + 1),
        _ => panic!("Invalid direction: {}", dir),
    }
}

pub fn get_opposite_direction(dir: char) -> char {
    match dir {
        '^' => 'v',
        'v' => '^',
        '<' => '>',
        '>' => '<',
        _ => panic!("Invalid direction: {}", dir),
    }
}

pub fn move_blocks_basic(warehouse: &mut Vec<Vec<char>>, pos: (usize, usize), new_pos: (usize, usize), dir: char) -> (usize, usize) {
    let mut new_block_pos = get_next_pos(new_pos, dir);
    let mut new_char = warehouse[new_block_pos.0][new_block_pos.1];

    // loop until we find an empty space or a wall
    while new_char != '.' && new_char != '#' {
        new_block_pos = get_next_pos(new_block_pos, dir);
        new_char = warehouse[new_block_pos.0][new_block_pos.1];
    }

    if new_char == '#' {
        // hit a wall, cannot move, return old position
        return pos;
    }

    // move the blocks, starting from the end and going to the start
    while new_block_pos != pos {
        let prev_pos = get_next_pos(new_block_pos, get_opposite_direction(dir));
        warehouse[new_block_pos.0][new_block_pos.1] = warehouse[prev_pos.0][prev_pos.1];
        new_block_pos = prev_pos;
    }

    return new_pos;
}

pub fn move_robot(warehouse: &mut Vec<Vec<char>>, pos: (usize, usize), dir: char) -> (usize, usize) {
    let new_pos = get_next_pos(pos, dir);

    let c = warehouse[new_pos.0][new_pos.1];

    if c == '#' {
        // hit a wall, cannot move, return old position
        return pos;
    }
    else if c == '.' {
        // empty space, move the robot
        return new_pos;
    }
    else if c == 'O' {
        // block, move the block and the robot
        return move_blocks_basic(warehouse, pos, new_pos, dir);
    }
    else {
        panic!("Invalid character: {}", c);
    }
}

pub fn move_robot_part2(warehouse: &mut Vec<Vec<char>>, pos: (usize, usize), dir: char) -> (usize, usize) {
    let new_pos = get_next_pos(pos, dir);

    let c = warehouse[new_pos.0][new_pos.1];

    if c == '#' {
        // hit a wall, cannot move, return old position
        return pos;
    }
    else if c == '.' {
        // empty space, move the robot
        return new_pos;
    }
    else if c == '[' || c == ']' {
        // block, move the block and the robot
        if dir == '<' || dir == '>' {
            // when going left/right, we can reuse out basic move function since the blocks are always in a single row
            return move_blocks_basic(warehouse, pos, new_pos, dir);
        }
        else {
            let mut min_col;
            let mut max_col;

            if c == '[' {
                min_col = new_pos.1;
                max_col = new_pos.1 + 1;
            }
            else {
                min_col = new_pos.1 - 1;
                max_col = new_pos.1;
            }

            let mut blocks_to_move: HashMap<usize, (usize, usize)> = HashMap::new(); // row -> (min_col, max_col, row)
            let mut found_boundary = false;
            let mut found_wall= false;
            let mut start_row = new_pos.0;
            let direction_modifier: i32 = if dir == 'v' { 1 } else { -1 };

            while !found_boundary {
                let next_row_chars = warehouse[start_row][min_col..=max_col].to_vec();

                // check if any char in the row is a wall
                if next_row_chars.iter().any(|c| *c == '#') {
                    found_boundary = true;
                    found_wall = true;
                }
                else if next_row_chars.iter().all(|c| *c == '.') {
                    // all empty spaces
                    found_boundary = true;
                }
                else {
                    // some boxes, store this and go one up
                    let mut min_char = warehouse[start_row][min_col];
                    let mut max_char = warehouse[start_row][max_col];

                    if min_char == ']' {
                        min_col -= 1;
                    }
                    else if min_char == '.' {
                        // .[]
                        // []
                        // boxes are at offset from previous, so we need to shrink the column until we find a non-empty space
                        while min_char == '.' {
                            min_col += 1;
                            min_char = warehouse[start_row][min_col];
                        }
                    }

                    if max_char == '[' {
                        max_col += 1;
                    }
                    else if max_char == '.' {
                        // boxes are at an offset from previous, so we need to shrink the column until we find a non-empty space
                        while max_char == '.' {
                            max_col -= 1;
                            max_char = warehouse[start_row][max_col];
                        }
                    }
                    
                    blocks_to_move.insert(start_row, (min_col, max_col));
                    start_row = (start_row as i32 + direction_modifier) as usize;
                }
            }

            if found_wall {
                return pos;
            }

            let keys = blocks_to_move.keys().sorted().collect::<Vec<&usize>>();
            if dir == '^' {
                // going up, copy blocks in that direction starting from the top
                for &key in keys.iter() {
                    let new_row = *key - 1;
                    let (min_col, max_col) = blocks_to_move.get(key).unwrap();
                    for col in *min_col..=*max_col {
                        warehouse[new_row][col] = warehouse[*key][col];
                        warehouse[*key][col] = '.';
                    }
                }
            }
            else {
                // going down, copy blocks in that direction starting from the bottom
                for &key in keys.iter().rev() {
                    let new_row = *key + 1;
                    let (min_col, max_col) = blocks_to_move.get(key).unwrap();
                    for col in *min_col..=*max_col {
                        warehouse[new_row][col] = warehouse[*key][col];
                        warehouse[*key][col] = '.';
                    }
                }
            }
            
            return new_pos;
        }
    }
    else {
        panic!("Invalid character: {}", c);
    }
}

pub fn _print_grid(warehouse: &Vec<Vec<char>>) {
    warehouse.iter().for_each(|line| {
        line.iter().for_each(|c| {
            print!("{}", c);
        });
        println!();
    });
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let (warehouse_lines, moves_line) = input.split("\n\n").collect_tuple().unwrap();

    let mut warehouse: Vec<Vec<char>> = warehouse_lines.lines().map(|line| line.chars().collect()).collect();

    // double the warehouse for part 2
    let mut warehouse_part2: Vec<Vec<char>> = Vec::new();
    warehouse.iter().for_each(|line| {
        let mut lines: Vec<char> = Vec::new();
        line.iter().enumerate().for_each(|(col, c)| {
            if *c == '#' {
                lines.push('#');
                lines.push('#');
            }
            else if *c == '.' {
                lines.push('.');
                lines.push('.');
            }
            else if *c == 'O' {
                lines.push('[');
                lines.push(']');
            }
            else if *c == '@' {
                lines.push('@');
                lines.push('.');
                
            }
        });

        warehouse_part2.push(lines);
    });

    let robot_pos = warehouse.iter().enumerate().find_map(|(y, line)| {
        line.iter().enumerate().find_map(|(x, c)| {
            if *c == '@' {
                Some((y, x))
            } else {
                None
            }
        })
    }).unwrap();

    let moves = moves_line.lines().join("").chars().collect::<Vec<char>>();
    let mut current_pos = robot_pos;
    
    moves.iter().for_each(|c| {
        let new_pos = move_robot(&mut warehouse, current_pos, *c);
        warehouse[current_pos.0][current_pos.1] = '.';
        warehouse[new_pos.0][new_pos.1] = '@';
        current_pos = new_pos;
    });

    warehouse.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, c)| {
            if *c == 'O' {
                result += 100 * row + col;
            }
        });
    });

    /////////////////////////
    ////// Part 2
    /////////////////////////

    let robot_pos = warehouse_part2.iter().enumerate().find_map(|(y, line)| {
        line.iter().enumerate().find_map(|(x, c)| {
            if *c == '@' {
                Some((y, x))
            } else {
                None
            }
        })
    }).unwrap();

    current_pos = robot_pos;
    moves.iter().for_each(|c| {
        let new_pos = move_robot_part2(&mut warehouse_part2, current_pos, *c);
        warehouse_part2[current_pos.0][current_pos.1] = '.';
        warehouse_part2[new_pos.0][new_pos.1] = '@';
        current_pos = new_pos;
    });

    warehouse_part2.iter().enumerate().for_each(|(row, line)| {
        line.iter().enumerate().for_each(|(col, c)| {
            if *c == '[' {
                result_part2 += 100 * row + col;
            }
        });
    });
    
    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}