use colored::Colorize;
use std::collections::{HashMap, HashSet};
use std::{thread, time};
use utils::read_input_to_string;

fn make_step(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 + b.0, a.1 + b.1)
}

fn step_back(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
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

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(6);
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    let mut dir_cycle = [(-1, 0), (0, 1), (1, 0), (0, -1)].iter().cycle();
    let mut dir_step = dir_cycle.next().unwrap();

    let mut cursors: Vec<char> = vec!['^', '>', 'v', '<'];
    let mut cursor_cycle = cursors.iter().cycle();
    let mut cur_cursor = cursor_cycle.next().unwrap();

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
            on_grid = false;
        } else {
            let next_char = grid
                .get(position.0 as usize)
                .unwrap()
                .get(position.1 as usize)
                .unwrap();

            if *next_char == '#' {
                position = step_back(position, *dir_step);
                dir_step = dir_cycle.next().unwrap();
                cur_cursor = cursor_cycle.next().unwrap();
            } else {
                visited.insert(position);
            }
        }
        show_grid(grid.clone(), position, *cur_cursor, None, Some(500 as u64));
    }
    visited.insert(starting_pos);
    score = visited.len() as i32;
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {}
