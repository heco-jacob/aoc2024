use colored::*;
use std::collections::{HashMap, HashSet};
use std::io;
use std::{thread, time};
use utils::read_input_to_string;

fn has_high_count(counts: &HashMap<String, usize>, threshold: usize) -> bool {
    counts.values().any(|&count| count >= threshold)
}
fn make_step(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn step_back(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
}

fn count_most_frequent<T: Eq + std::hash::Hash>(vec: &Vec<T>) -> usize {
    let mut counts = HashMap::new();

    // Count occurrences of each item
    for item in vec {
        *counts.entry(item).or_insert(0) += 1;
    }

    // Find the maximum occurrence count
    counts.values().cloned().max().unwrap_or(0)
}

fn show_grid2(mut grid: Vec<Vec<char>>, guard_position: (i32, i32), cursor: char) {
    grid[guard_position.0 as usize][guard_position.1 as usize] = cursor;
    for row in &grid {
        println!("{}", row.iter().collect::<String>());
    }
}

fn show_grid(
    grid: Vec<Vec<char>>,
    guard_position: (i32, i32),
    cursor: char,
    obstacle_position: Option<(i32, i32)>, // allows me to highlight the obstacle with colored library
    sleep: Option<u64>,
) {
    if let Some(duration) = sleep {
        let sleep_duration = time::Duration::from_millis(duration);
        thread::sleep(sleep_duration);
    }

    let (guard_x, guard_y) = guard_position;

    for (i, row) in grid.iter().enumerate() {
        let highlighted_row: String = row
            .iter()
            .enumerate()
            .map(|(j, &c)| {
                if i == guard_x as usize && j == guard_y as usize {
                    format!("{}", cursor.to_string().red()) // colored demands string
                } else if let Some((obs_x, obs_y)) = obstacle_position {
                    if i == obs_x as usize && j == obs_y as usize {
                        format!("{}", c.to_string().blue()) // colored demands string
                    } else {
                        format!("{}", c)
                    }
                } else {
                    format!("{}", c)
                }
            })
            .collect();

        println!("{}", highlighted_row);
    }
}

fn simulate(grid: Vec<Vec<char>>, obstacle_pos: (i32, i32)) -> i32 {
    let mut dir_cycle = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
    let mut dir_step = dir_cycle.next().unwrap();

    let mut position: (i32, i32) = (0, 0);
    // find starting position
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                position = (i as i32, j as i32);
            }
        }
    }
    let starting_pos = position.clone();

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_vec: Vec<(i32, i32)> = Vec::new();
    let mut corner_counts: HashMap<String, usize> = HashMap::new();

    let mut cursors: Vec<char> = vec!['^', '>', 'v', '<'];
    let mut cursor_cycle = cursors.iter().cycle();
    let mut cur_cursor = cursor_cycle.next().unwrap();

    let column_boundary = grid[0].len() as i32;
    let row_boundary = grid.len() as i32;

    let mut on_grid = true;
    while on_grid {
        if has_high_count(&corner_counts, 2) {
            return 1;
        }

        position = make_step(position, *dir_step);
        if position.0 < 0
            || position.0 >= row_boundary
            || position.1 < 0
            || position.1 >= column_boundary
        {
            let one_off_position = step_back(position, *dir_step);
            visited_vec.push(one_off_position);
            on_grid = false;
        } else {
            let next_char = grid
                .get(position.0 as usize)
                .unwrap()
                .get(position.1 as usize)
                .unwrap();

            if *next_char == '#' {
                position = step_back(position, *dir_step);
                visited_vec.push(position);
                let corner = format!("{}-{}-{}", position.0, position.1, cur_cursor); // store corner with direction
                *corner_counts.entry(corner).or_insert(0) += 1;
                dir_step = dir_cycle.next().unwrap(); // move right
                cur_cursor = cursor_cycle.next().unwrap(); // switch cursor for grid animation
            } else {
                visited_vec.push(position);
                visited.insert(position);

                // show_grid(
                //     grid.clone(),
                //     position,
                //     *cur_cursor,
                //     Some(obstacle_pos),
                //     Some(80 as u64),
                // );
            }
        }
    }
    visited_vec.push(starting_pos);
    visited.insert(starting_pos);

    return 0; // Doesn't count towards part 2 count
}

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(6);

    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut dir_cycle = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
    let mut dir_step = dir_cycle.next().unwrap();

    let mut position: (i32, i32) = (0, 0);
    for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == '^' {
                position = (i as i32, j as i32);
            }
        }
    }
    let starting_pos = position.clone();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut visited_vec: Vec<(i32, i32)> = Vec::new();

    let mut cursors: Vec<char> = vec!['^', '>', 'v', '<'];
    let mut cursor_cycle = cursors.iter().cycle();
    let mut cur_cursor = cursor_cycle.next().unwrap();

    let column_boundary = grid[0].len() as i32;
    let row_boundary = grid.len() as i32;

    let mut on_grid = true;
    while on_grid {
        position = make_step(position, *dir_step);
        if position.0 < 0
            || position.0 >= row_boundary
            || position.1 < 0
            || position.1 >= column_boundary
        {
            let one_off_position = step_back(position, *dir_step);
            visited_vec.push(one_off_position);

            on_grid = false;
        } else {
            let next_char = grid
                .get(position.0 as usize)
                .unwrap()
                .get(position.1 as usize)
                .unwrap();

            if *next_char == '#' {
                position = step_back(position, *dir_step);
                visited_vec.push(position);
                dir_step = dir_cycle.next().unwrap();
                cur_cursor = cursor_cycle.next().unwrap();
            } else {
                visited_vec.push(position);
                visited.insert(position);
                // show_grid(grid.clone(), position, *cur_cursor, None, None);
            }
        }
    }
    visited_vec.push(starting_pos);
    visited.insert(starting_pos);

    // brute force:

    let mut used = Vec::new();
    let unique_pairs: HashSet<_> = visited_vec.into_iter().collect();

    println!("Number of possible nodes {:?}", unique_pairs.len());
    for (i, node) in unique_pairs.iter().enumerate() {
        if i % 100 == 0 {
            println!("One line number {i} processing {node:?}");
        }
        let mut my_grid = grid.clone();

        let i = node.0 as usize;
        let j = node.1 as usize;

        if used.contains(node) {
            continue;
        }
        my_grid[i][j] = '#';
        score += simulate(my_grid, (i as i32, j as i32));
    }
    println!("Part 2: {}", score);
}

#[cfg(test)]
mod tests {}
