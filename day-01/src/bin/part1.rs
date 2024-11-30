use std::fs;
use std::env;
use crate::utils::say_hello;


mod utils;

const INPUT_DIR: &str = "../inputs"; // Adjust the relative path as needed


fn main() {
    utils::say_hello();
    let file_path = format!("{}/day1_input.txt", INPUT_DIR);
    let input = fs::read_to_string(file_path).expect("Could not read file");

    for line in input.lines() {
        println!("{}", line);
    }
}
