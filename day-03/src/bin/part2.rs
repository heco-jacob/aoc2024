use regex::{Captures, Regex};
use utils::read_input_to_string;

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(3);

    // mut(x,y)
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let muts: Vec<_> = re.captures_iter(&input).collect();
    // don't()
    let re2 = Regex::new(r"(don't\(\))()").unwrap();
    let donts: Vec<_> = re2.captures_iter(&input).collect();
    // do()
    let re3 = Regex::new(r"(do\(\))()").unwrap();
    let dos: Vec<Captures> = re3.captures_iter(&input).collect();

    // one big happy family
    let mut all_captures: Vec<Captures> = Vec::new();
    all_captures.extend(muts);
    all_captures.extend(donts);
    all_captures.extend(dos);
    // getting clever
    all_captures.sort_by_key(|cap| cap.get(1).map(|m| m.start()));

    let mut multi_party = true;

    for cap in all_captures {
        if &cap[0] == "don't()" {
            multi_party = false;
        } else if &cap[0] == "do()" {
            multi_party = true;
        } else if multi_party {
            let f = cap[1].parse::<i32>().unwrap();
            let s = cap[2].parse::<i32>().unwrap();
            score += f * s;
        }
    }
    println!("{}", score);
}

#[cfg(test)]
mod tests {}
