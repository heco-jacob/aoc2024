use utils::read_input_to_string;
use std::collections::{HashSet, HashMap};

fn get_distance(x: (i32,i32), y: (i32,i32)) -> i32 {
    (x.0 - y.0).abs() + (x.1 - y.1).abs()
}

fn main() {
    let mut score: i64 = 0;
    let input = read_input_to_string(8);
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut antenna_types: HashSet<char> =  HashSet::new();
    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

    // find antenna types
    for i in 0..grid.len() {
        for j in 00..grid.len() {
            if antennas.contains_key(&grid[i][j]) {
                antennas.entry(grid[i][j]).or_insert_with(Vec::new).push((i, j));
                continue;
            }
            else {
                let mut indices = Vec::new();
                antennas.insert(grid[i][j], indices);
                antennas.entry(grid[i][j]).or_insert_with(Vec::new).push((i, j));

            }

        }
    }

    // find location of each antenna types
    for i in antennas.keys() {
        println!("key: {:?}", i);
        println!("{:?}", antennas.get(i).unwrap());
    }

    


    let a = vec![1,2,3];
    let mut b = a.iter().cycle();


    println!("Final Score: {}", score);
}
