use std::collections::{HashMap, HashSet};
use utils::read_input_to_string;

fn get_vector(from: (i32, i32), to: (i32, i32)) -> (i32, i32) {
    (from.0 - to.0, from.1 - to.1)
}

fn add_vector(x: (i32, i32), y: (i32, i32)) -> (i32, i32) {
    (x.0 + y.0, x.1 + y.1)
}

fn main() {
    let mut score: i64 = 0;
    let input = read_input_to_string(8);
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // find antenna types
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for i in 0..grid.len() {
        for j in 00..grid.len() {
            let index: (i32, i32) = (i as i32, j as i32);
            let antenna = grid[i][j];
            if antenna == '.' {
                continue;
            }
            antennas.entry(antenna).or_insert_with(Vec::new).push(index);
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for a_type in antennas.iter() {
        for coord in &mut *a_type.1.clone() {
            for other_coord in &*a_type.1.clone() {
                if coord != other_coord {
                    let vector = get_vector(*coord, *other_coord);
                    let node = add_vector(*coord, vector);
                    if node.0 > grid.len() as i32 - 1
                        || node.1 > grid[0].len() as i32 - 1
                        || node.0 < 0
                        || node.1 < 0
                    {
                        continue;
                    } else {
                        antinodes.insert(node);
                    }
                }
            }
        }
    }

    score = antinodes.len() as i64;
    println!("Part 1: {}", score);
}
