use utils::read_input_to_string;

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(5);

    let parts: Vec<&str> = input.split("\r\n\r\n").collect();

    // page ordering rules
    let por_section = parts[0];
    // page numbers update
    let pnu_section = parts[1];

    let mut por_vec: Vec<Vec<i32>> = Vec::new();
    let mut pnu_vec: Vec<Vec<i32>> = Vec::new();

    for line in por_section.lines() {
        let rule: Vec<&str> = line.split("|").collect();

        if let (Ok(first), Ok(second)) = (rule[0].parse::<i32>(), rule[1].parse::<i32>()) {
            let por_vector = vec![first, second];
            por_vec.push(por_vector);
        }
    }

    for line in pnu_section.lines() {
        let pnu_vector: Vec<i32> = line
            .split(',')
            .map(|num| num.trim())
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();
        pnu_vec.push(pnu_vector);
    }

    for update_vec in pnu_vec {
        let mut go = true;
        for rule in &por_vec {
            if update_vec.contains(&rule[0]) && update_vec.contains(&rule[1]) {
                let idx1 = update_vec.iter().position(|x| x == &rule[0]).unwrap();
                let idx2 = update_vec.iter().position(|x| x == &rule[1]).unwrap();
                if idx1 > idx2 {
                    go = false;
                    // println!("Update vec {:?} is invalid", update_vec);
                    break;
                }
            }
        }
        if go {
            // println!("Update vec {:?} is valid", update_vec);
            let mid_idx = update_vec.len() / 2;
            score += update_vec[mid_idx];
        }
    }
    println!("Part 1: {}", score);
}

#[cfg(test)]
mod tests {}
