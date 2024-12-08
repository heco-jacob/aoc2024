use utils::read_input_to_string;

fn check_validity(vec: &Vec<i32>) -> bool {
    let s_valid = vec
        .windows(2)
        .map(|x| (x[1] - x[0]))
        .filter(|x| 3 >= *x && *x > 0)
        .count();
    let rs_valid = vec
        .windows(2)
        .map(|x| (x[0] - x[1]))
        .filter(|x| 3 >= *x && *x > 0)
        .count();
    if s_valid == vec.len() - 1 || rs_valid == vec.len() - 1 {
        true
    } else {
        false
    }
}
fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(2);
    for line in input.lines() {
        let mut vec: Vec<i32> = line.split(" ").map(|x| x.parse::<i32>().unwrap()).collect();
        if check_validity(&vec) {
            score = score + 1;
            // println!("Safe: {:?}", &vec);
        } else {
            // println!("Unsafe: {:?}", &vec);
        }
    }
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {}
