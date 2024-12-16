use utils::read_input_to_string;

fn sequence_to_string(seq: &Vec<(Option<i32>, i32, bool)>) -> String {
    let mut result = String::new();

    for (id, size, is_file) in seq.iter() {
        if *is_file {
            // File blocks are represented by their ID
            let char_rep = id.unwrap_or(0).to_string(); // Use 0 as default if no ID
            result.push_str(&char_rep.repeat(*size as usize));
        } else {
            result.push_str(&".".repeat(*size as usize));
        }
    }
    result
}

fn main() {
    let mut score: i64 = 0;
    let input: String = read_input_to_string(9);
    let charvec: Vec<char> = input.chars().collect();

    let mut seq: Vec<(Option<i32>, i32, bool)> = vec![]; // (id: i32, size: i32, is_file: bool)

    let mut id = 0;
    for (idx, c) in charvec.iter().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if idx % 2 == 0 {
                seq.push((Some(id), digit as i32, true));
                id += 1;
            } else {
                seq.push((None, digit as i32, false));
            }
        }
    }

    let mut file_ids: Vec<_> = seq
        .iter()
        .filter(|(_, _, is_file)| *is_file)
        .filter_map(|(id, _, _)| *id)
        .collect();

    for id in file_ids.iter().rev() {
        println!("String: {}", sequence_to_string(&seq));

        if let Some(file_index) = seq.iter().position(|(seq_id, _, _)| seq_id == &Some(*id)) {
            let file = seq[file_index];
            if let Some(space_index) = seq
                .iter()
                .position(|(seq_id, size, is_file)| size >= &file.1 && !is_file)
            {
                let space = seq[space_index];
                // println!("File: {:?} at file index {:?}, space: {:?} at space index {:?}", file, file_index, space, space_index);
                if space_index > file_index {
                    continue;
                }
                seq[space_index] = file;
                seq[file_index] = (None, file.1, false);
                let diff = (file.1 - space.1).abs();
                if diff > 0 {
                    // println!("Diff is {:?}, adding new space", diff);
                    let new_space: (Option<i32>, i32, bool) = (None, diff, false);
                    seq.insert(space_index + 1, new_space);
                }
            } else {
            }
        }
    }

    // scoring
    let s = sequence_to_string(&seq);
    for (idx, c) in s.chars().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            let add = digit as i64 * idx as i64;
            score += add;
        }
    }

    println!("Part 2: {}", score);
}
