use itertools::{iproduct, Itertools};
use utils::read_input_to_string;

fn mul(a: i32, b: i32) -> i32 {
    a * b
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}


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

            let target: i64 = parts[0].trim().parse::<i64>().expect("Failed to parse the number");

            let values: Vec<i64> = parts[1]
                .split_whitespace()
                .filter_map(|s| s.parse::<i64>().ok())
                .collect();

            if values.len() < 2 {
                println!("Need at least two values to apply functions.");
                continue;
            }

            // Generate all possible combinations of functions for (values.len() - 1)
            let func_combinations = (0..values.len() - 1)
                .map(|_| functions.iter())
                .multi_cartesian_product();

            for func_combination in func_combinations {
                let mut iter = values.iter().copied();
                let mut result: i64 = iter.next().unwrap(); // Start with the first number
                if run {
                    for (value, func) in iter.zip(func_combination) {
                        result = func.apply(result, value);
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

    println!("Final Score: {}", score);
}
