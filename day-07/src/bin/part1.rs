use itertools::Itertools;
use utils::read_input_to_string;

use std::fmt;

enum NamedFunction {
    Add,
    Mul,
}

impl NamedFunction {
    fn apply(&self, x: i64, y: i64) -> i64 {
        match self {
            NamedFunction::Add => x + y,
            NamedFunction::Mul => x * y,
        }
    }
}

// see example from chess engine
impl fmt::Debug for NamedFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamedFunction::Add => write!(f, "add"),
            NamedFunction::Mul => write!(f, "mul"),
        }
    }
}

fn main() {
    let mut score: i64 = 0;
    let input = read_input_to_string(7);
    let functions: Vec<NamedFunction> = vec![NamedFunction::Add, NamedFunction::Mul];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() == 2 {
            let mut run = true;
            let target: i64 = parts[0]
                .trim()
                .parse::<i64>()
                .expect("Failed to parse the number");
            let values: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();

            let func_combinations = (0..values.len() - 1)
                .map(|_| functions.iter())
                .multi_cartesian_product(); // creates a MultiProduct iterator, that can be collected to inspect

            // like this:
            // let collection = func_combinations.clone().collect::<Vec<_>>();

            for func_combination in func_combinations {
                let mut result = values[0];
                let mut iter = values.iter().skip(1);
                if run {
                    for (value, func) in iter.zip(func_combination) {
                        result = func.apply(result, *value);
                        // Compare with target
                        if result == target {
                            score += result;
                            run = false;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", score);
}
