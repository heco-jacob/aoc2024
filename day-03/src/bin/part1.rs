use regex::Regex;
use utils::read_input_to_string;

// xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(3);

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let captures: Vec<_> = re.captures_iter(&input).collect();
    for cap in captures.iter() {
        let f: i32 = cap[1].parse().unwrap();
        let s: i32 = cap[2].parse().unwrap();
        score += f * s;
    }
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {}
