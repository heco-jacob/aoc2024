use std::collections::{HashMap, HashSet};
use utils::read_input_to_string;

fn get_vector(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    (from.0 - to.0, from.1 - to.1)
}

fn add_vector(x: (i32, i32), y: (i32, i32)) -> (i32, i32) {
    (x.0 + y.0, x.1 + y.1)
}

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(8);
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    // find antenna types
    for i in 0..grid.len() {
        for j in 0..grid.len() {
            let index: (i32, i32) = (i as i32, j as i32);
            let antenna = grid[i][j];
            if antenna == '.' {
                continue;
            }
            antennas.entry(antenna).or_insert_with(Vec::new).push(index);
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
    let mut new_grid = grid.clone();
    // find location of each antenna types
    for a_type in antennas.iter() {
        for coord in a_type.1.clone() {
            for other_coord in a_type.1.clone() {
                if coord != other_coord {
                    let vector = get_vector(coord, other_coord);
                    let mut run = true;
                    let mut node = add_vector(coord, vector);

                    while !(node.0 > grid.len() as i32 - 1
                        || node.1 > grid[0].len() as i32 - 1
                        || node.0 < 0
                        || node.1 < 0)
                        && run
                    {
                        if antinodes.contains(&node) {
                            let vector = get_vector(coord, other_coord);
                            node = add_vector(node, vector);
                            continue;
                        } else {
                            antinodes.insert(node);
                            new_grid[node.0 as usize][node.1 as usize] = '#';
                            node = add_vector(node, vector);
                        }
                    }
                }
            }
        }
    }

    for i in antennas.iter() {
        for j in i.1.clone() {
            if antinodes.contains(&j) {
                continue;
            } else {
                score += 1;
            }
        }
    }
    score += antinodes.len() as i32;
    println!("Final Score: {}", score);
}
