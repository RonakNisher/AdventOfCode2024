
use std::collections::{BinaryHeap, HashMap, HashSet};
#[derive(Eq, PartialEq, Debug, Copy, Clone, Hash)]
pub enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Eq, PartialEq, Debug, Hash)]
struct State {
    cost: i32,
    position: (usize, usize),
    direction: Dir,
    path: Vec<(usize, usize)>
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.cmp(&other.cost).reverse()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn turn_left(dir: &Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Left,
		Dir::Down => Dir::Right,
		Dir::Left => Dir::Down,
		Dir::Right => Dir::Up,
	}
}

fn turn_right(dir: &Dir) -> Dir {
	match dir {
		Dir::Up => Dir::Right,
		Dir::Down => Dir::Left,
		Dir::Left => Dir::Up,
		Dir::Right => Dir::Down,
	}
}

fn advance_pos(pos: (usize, usize), dir: &Dir) -> (usize, usize) {
    match dir {
     Dir::Up => (pos.0 - 1, pos.1),
     Dir::Down => (pos.0 + 1, pos.1),
     Dir::Left => (pos.0, pos.1 - 1),
     Dir::Right => (pos.0, pos.1 + 1),
    }
}

pub fn solve_maze_bfs(maze: &Vec<Vec<char>>, start: (usize, usize), end: (usize, usize)) -> (i32, i32) {
    let mut priority_queue = BinaryHeap::new();
    let mut visited: HashMap<((usize, usize), Dir), i32> = HashMap::new(); // pos -> cost
    let mut best_path: HashSet<(usize, usize)> = HashSet::new();
    let mut min_cost: Option<i32> = None;

    priority_queue.push(State {
        cost: 0,
        position: start,
        direction: Dir::Right,
        path: vec![start],
    });

    while let Some(State { cost, position, direction , path}) = priority_queue.pop() {
        if maze[position.0][position.1] == '#' {
            continue;
        }
        
        if position == end {
            if min_cost.is_none() || cost <= min_cost.unwrap() {
                min_cost = Some(cost);
                best_path.extend(path.iter());
            }
        }

        if let Some(&prev_cost) = visited.get(&(position, direction)) {
            if cost > prev_cost {
                continue;
            }
        }
        else {
            visited.insert((position, direction), cost);
        }

        // left
        priority_queue.push(State {
            cost: cost + 1000,
            position: position,
            direction: turn_left(&direction),
            path: path.clone(),
        });

        // right
        priority_queue.push(State {
            cost: cost + 1000,
            position: position,
            direction: turn_right(&direction),
            path: path.clone(),
        });

        // forward
        let new_position = advance_pos(position, &direction);
        let mut new_path = path.clone();
        new_path.push(new_position);
        priority_queue.push(State {
            cost: cost + 1,
            position: new_position,
            direction: direction,
            path: new_path,
        });
    }

    return (min_cost.unwrap(), best_path.len() as i32);
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

    (result, result_part2) = solve_maze_bfs(&maze, start, end);
    
    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}