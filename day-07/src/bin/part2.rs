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
    Concat,
}

impl NamedFunction {
    fn apply(&self, x: i64, y: i64) -> i64 {
        match self {
            NamedFunction::Add => x + y,
            NamedFunction::Mul => x * y,
            NamedFunction::Concat => {
                let concatenated = format!("{}{}", x, y); // Convert both numbers to strings and concatenate
                concatenated.parse::<i64>().unwrap() // Parse the concatenated string back to i64
            }
        }
    }
}

impl fmt::Debug for NamedFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NamedFunction::Add => write!(f, "add"),
            NamedFunction::Mul => write!(f, "mul"),
            NamedFunction::Concat => write!(f, "concat"),
        }
    }
}

fn main() {
    let mut score: i64 = 0;
    let input = read_input_to_string(7);

    let functions: Vec<NamedFunction> = vec![NamedFunction::Add, NamedFunction::Mul, NamedFunction::Concat];

    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            println!("Invalid input line: {}", line);
            continue;
        }

        let target: i64 = parts[0].trim().parse::<i64>().expect("Failed to parse target value");
        let values: Vec<i64> = parts[1]
            .split_whitespace()
            .filter_map(|s| s.parse::<i64>().ok())
            .collect();

        if values.len() < 2 {
            println!("Insufficient values in line: {}", line);
            continue;
        }

        let func_combinations = (0..values.len() - 1)
            .map(|_| functions.iter())
            .multi_cartesian_product();

        let mut found = false;
        for func_combination in func_combinations {
            let mut result = values[0]; // Start with the first value
            let mut iter = values.iter().skip(1); // Skip the first value

            for (value, func) in iter.zip(func_combination) {
                result = func.apply(result, *value);
            }

            if result == target {
                score += result;
                found = true;
                break; // Early exit once a valid combination is found
            }
        }

        if !found {
            println!("No valid combination found for target: {}", target);
        }
    }

    println!("Final Score: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_funcs(){
        assert_eq!(NamedFunction::Concat.apply(1, 2),12);
        assert_eq!(NamedFunction::Mul.apply(1, 2),2);
        assert_eq!(NamedFunction::Add.apply(2, 2),4);
        assert_eq!(NamedFunction::Concat.apply(1, 10),110);
        assert_eq!(NamedFunction::Concat.apply(200, 200),200200);
        assert_eq!(NamedFunction::Concat.apply(12125, 73273),1212573273);


    }
}