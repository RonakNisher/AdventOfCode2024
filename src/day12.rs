use std::collections::{HashMap, HashSet};

use itertools::Itertools;

pub fn get_sides(points: &HashSet<(i32, i32)>) -> i32 {
    // each plant point has 4 possible sides, so clone the plants for each side and then reduce them
    let mut top = points.clone();
    let mut bottom = points.clone();
    let mut left = points.clone();
    let mut right = points.clone();

    // all internal points on that side are removed e.g. if the point at the top is the same as the current one, remove it
    top.retain(|(x, y)| !points.contains(&(x - 1, *y)));
    bottom.retain(|(x, y)| !points.contains(&(x + 1, *y)));
    left.retain(|(x, y)| !points.contains(&(*x, y - 1)));
    right.retain(|(x, y)| !points.contains(&(*x, y + 1)));

    // now reduce the points for each side, all contiguous points in the same row/col are reduced to 1

    // top
    let mut top_rows_sorted = top.iter().sorted().collect_vec();

    let mut i = 0;
    let mut indexes_to_be_removed: Vec<usize> = Vec::new();
    while i < top_rows_sorted.len() {
        let mut j = i + 1;
        let mut max_col = top_rows_sorted[i].1;
        while j < top_rows_sorted.len() && top_rows_sorted[i].0 == top_rows_sorted[j].0 && top_rows_sorted[j].1 == max_col + 1 {
            indexes_to_be_removed.push(j);
            max_col = top_rows_sorted[j].1;
            j += 1;
        }
        i = j;
    }

    for i in indexes_to_be_removed.iter().rev() {
        top_rows_sorted.remove(*i);
    }

    // bottom
    let mut bottom_rows_sorted = bottom.iter().sorted().collect_vec();

    let mut i = 0;
    let mut indexes_to_be_removed: Vec<usize> = Vec::new();
    while i < bottom_rows_sorted.len() {
        let mut j = i + 1;
        let mut max_col = bottom_rows_sorted[i].1;
        while j < bottom_rows_sorted.len() && bottom_rows_sorted[i].0 == bottom_rows_sorted[j].0 && bottom_rows_sorted[j].1 == max_col + 1 {
            indexes_to_be_removed.push(j);
            max_col = bottom_rows_sorted[j].1;
            j += 1;
        }
        i = j;
    }

    for i in indexes_to_be_removed.iter().rev() {
        bottom_rows_sorted.remove(*i);
    }

    // left
    let mut left_cols_sorted = left.iter().sorted_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0))).collect_vec();

    let mut i = 0;
    let mut indexes_to_be_removed: Vec<usize> = Vec::new();
    while i < left_cols_sorted.len() {
        let mut j = i + 1;
        let mut max_row = left_cols_sorted[i].0;
        while j < left_cols_sorted.len() && left_cols_sorted[i].1 == left_cols_sorted[j].1 && left_cols_sorted[j].0 == max_row + 1 {
            indexes_to_be_removed.push(j);
            max_row = left_cols_sorted[j].0;
            j += 1;
        }
        i = j;
    }

    for i in indexes_to_be_removed.iter().rev() {
        left_cols_sorted.remove(*i);
    }

    // right
    let mut right_cols_sorted = right.iter().sorted_by(|a, b| a.1.cmp(&b.1).then(a.0.cmp(&b.0))).collect_vec();

    let mut i = 0;
    let mut indexes_to_be_removed: Vec<usize> = Vec::new();
    while i < right_cols_sorted.len() {
        let mut j = i + 1;
        let mut max_row = right_cols_sorted[i].0;
        while j < right_cols_sorted.len() && right_cols_sorted[i].1 == right_cols_sorted[j].1 && right_cols_sorted[j].0 == max_row + 1 {
            indexes_to_be_removed.push(j);
            max_row = right_cols_sorted[j].0;
            j += 1;
        }
        i = j;
    }

    for i in indexes_to_be_removed.iter().rev() {
        right_cols_sorted.remove(*i);
    }

    // all that's left are the sides that cannot be reduced any further

    return top_rows_sorted.len() as i32 + bottom_rows_sorted.len() as i32 + left_cols_sorted.len() as i32 + right_cols_sorted.len() as i32;
}

pub fn get_perimeter(grid: &Vec<Vec<char>>, x: i32, y: i32, current_region: char) -> u32 {
    let mut perimeter = 0;

    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    directions.iter().for_each(|(dx, dy)| {
        let new_x = x as i32 + dx;
        let new_y = y as i32 + dy;

        if new_x < 0 || new_y < 0 || new_x >= grid.len() as i32 || new_y >= grid[0].len() as i32 {
            perimeter += 1;
        } 
        else 
        {
            if grid[new_x as usize][new_y as usize] != current_region {
                perimeter += 1;
            }
        }
    });

    return perimeter;
}

pub fn get_region_area_and_parameter(grid: &Vec<Vec<char>>, x: i32, y: i32, current_region: char, visited: &mut HashSet<(i32, i32)>) -> (u32, u32) {
    
    if x < 0 || y < 0 || x >= grid.len() as i32 || y >= grid[0].len() as i32{
        return (0, 0);
    }
    
    let plant = grid[x as usize][y as usize];
    
    if visited.contains(&(x, y)) || plant != current_region {
        return (0, 0);
    }
    
    visited.insert((x, y));
            
    let mut area = 1;
    let mut perimeter = get_perimeter(grid, x, y, current_region);

    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    directions.iter().for_each(|(dx, dy)| {
        let (a, p) = get_region_area_and_parameter(grid, x + dx, y + dy, current_region, visited);
        area += a;
        perimeter += p;
    });

    return (area, perimeter);
}

pub fn solve(input: String) {
    let mut result = 0;
    let mut result_part2 = 0;

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut plants_visited: HashSet<(i32, i32)> = HashSet::new();

    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if !plants_visited.contains(&(i as i32, j as i32)) {
                let mut visited = HashSet::new();
                let (area, perimeter) = get_region_area_and_parameter(&grid, i as i32, j as i32, grid[i][j], &mut visited);
                let sides = get_sides(&visited);

                plants_visited.extend(visited);

                result += area * perimeter;
                result_part2 += area * sides as u32;
            }
        }
    }

    println!("*******************");
    println!("Solved Day 1 Part 1: {}", result);
    println!("Solved Day 1 Part 2: {}", result_part2);
    println!("*******************");
}