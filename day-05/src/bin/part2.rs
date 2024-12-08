use std::collections::{HashMap, HashSet};
use utils::read_input_to_string;

fn reorder_update(update: &Vec<i32>, por_vec: &Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut in_degree: HashMap<i32, usize> = HashMap::new();

    for rule in por_vec {
        if update.contains(&rule[0]) && update.contains(&rule[1]) {
            graph.entry(rule[0]).or_default().push(rule[1]);
            *in_degree.entry(rule[1]).or_default() += 1;
            in_degree.entry(rule[0]).or_default();
        }
    }
    let mut zero_in_degree: Vec<i32> = update
        .iter()
        .filter(|&&node| in_degree.get(&node).copied().unwrap_or(0) == 0)
        .cloned()
        .collect();

    let mut sorted = vec![];

    while let Some(node) = zero_in_degree.pop() {
        sorted.push(node);
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        zero_in_degree.push(neighbor);
                    }
                }
            }
        }
    }

    sorted
}

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
        let mut valid = true;
        for rule in &por_vec {
            if update_vec.contains(&rule[0]) && update_vec.contains(&rule[1]) {
                let idx1 = update_vec.iter().position(|x| x == &rule[0]).unwrap();
                let idx2 = update_vec.iter().position(|x| x == &rule[1]).unwrap();
                if idx1 > idx2 {
                    valid = false;
                    break;
                }
            }
        }
        if valid {
            // println!("Update vec {:?} is valid", update_vec);
        } else {
            // println!("Update vec {:?} is invalid", update_vec);
            let sorted_update = reorder_update(&update_vec, &por_vec);
            let mid_idx = sorted_update.len() / 2;
            score += sorted_update[mid_idx];
        }
    }
    println!("Part 2: {}", score);
}

#[cfg(test)]
mod tests {}
