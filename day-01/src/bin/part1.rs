use utils::read_input_to_string;

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(1);
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

    list_one.sort();
    list_two.sort();

    for (item_one, item_two) in list_one.iter().zip(&list_two) {
        score += (item_one - item_two).abs();
    }
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {}
