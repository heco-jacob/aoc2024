mod utils;
use crate::utils::read_input_to_string;

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(1, Some(false));
    let mut list_one: Vec<i32> = Vec::new();
    let mut list_two: Vec<i32> = Vec::new();

    for l in input.lines() {
        let line: Vec<&str> = l.split_whitespace().collect();
        if let [first, second] = &line[..] {
            let first: i32 = first.parse().unwrap();
            let second: i32 = second.parse().unwrap();
            list_one.push(first);
            list_two.push(second);
        }
    }

    for i in list_one.iter() {
        score += i * (list_two.iter().filter(|&&x| x == *i).count() as i32);
    }

    println!("{}", score);
}

#[cfg(test)]
mod tests {}
