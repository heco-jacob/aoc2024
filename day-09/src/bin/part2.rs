use utils::read_input_to_string;

fn sequence_to_string(seq: &Vec<(Option<i32>, i32, bool)>) -> String {
    let mut result = String::new();

    for (id, size, is_file) in seq.iter() {
        if *is_file {
            let char_rep = id.unwrap_or(0).to_string();
            result.push_str(&char_rep.repeat(*size as usize));
        } else {
            result.push_str(&".".repeat(*size as usize));
        }
    }
    result
}

fn diskmap_to_files(char_vec: Vec<char>) -> Vec<(Option<i32>, i32, bool)> {
    let mut seq: Vec<(Option<i32>, i32, bool)> = vec![]; // (id: i32, size: i32, is_file: bool)
    let mut id = 0;
    for (idx, c) in char_vec.iter().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if idx % 2 == 0 {
                seq.push((Some(id), digit as i32, true));
                id += 1;
            } else {
                seq.push((None, digit as i32, false));
            }
        }
    }
    seq
}

fn to_char_vec(input: &str) -> Vec<char> {
    input.chars().collect()
}

fn scoring(seq: &Vec<(Option<i32>, i32, bool)>) -> i64 {
    let mut score: i64 = 0;
    let mut index = 0;
    for (id, size, _) in seq.iter() {
        if let Some(id_int) = *id {
            for _ in 0..*size {
                score += id_int as i64 * index as i64;
                index += 1;
            }
        } else {
            index += size;
        }
    }
    score
}

fn main() {
    let mut score: i64 = 0;
    let input: String = read_input_to_string(9);
    let char_vec: Vec<char> = to_char_vec(&input);

    let mut seq = diskmap_to_files(char_vec);

    let mut file_ids: Vec<_> = seq
        .iter()
        .filter(|(_, _, is_file)| *is_file)
        .filter_map(|(id, _, _)| *id)
        .collect();

    for id in file_ids.iter().rev() {
        // println!("String: {}", sequence_to_string(&seq));

        if let Some(file_index) = seq.iter().position(|(seq_id, _, _)| seq_id == &Some(*id)) {
            let file = seq[file_index];
            if let Some(space_index) = seq
                .iter()
                .position(|(seq_id, size, is_file)| size >= &file.1 && !is_file)
            {
                let space = seq[space_index];
                if space_index > file_index {
                    continue;
                }
                seq[space_index] = file;
                seq[file_index] = (None, file.1, false);
                let diff = (file.1 - space.1).abs();
                if diff > 0 {
                    let new_space: (Option<i32>, i32, bool) = (None, diff, false);
                    seq.insert(space_index + 1, new_space);
                }
            } else {
            }
        }
    }
    score += scoring(&seq);
    println!("Part 2: {}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_to_char_vec() {
        let string = "1234";
        let char_vec: Vec<char> = to_char_vec(string);
        assert_eq!(char_vec, vec!['1', '2', '3', '4']);
    }

    #[test]
    fn test_diskmap_to_files() {
        let char_vec: Vec<char> = to_char_vec("12345");
        let files = diskmap_to_files(char_vec);
        assert_eq!(
            files,
            vec![
                (Some(0), 1, true),
                (None, 2, false),
                (Some(1), 3, true),
                (None, 4, false),
                (Some(2), 5, true)
            ]
        );

        let char_vec: Vec<char> = to_char_vec("101");
        let files = diskmap_to_files(char_vec);
        assert_eq!(
            files,
            vec![(Some(0), 1, true), (None, 0, false), (Some(1), 1, true)]
        );

        let char_vec: Vec<char> = to_char_vec("010");
        let files = diskmap_to_files(char_vec);
        assert_eq!(
            files,
            vec![(Some(0), 0, true), (None, 1, false), (Some(1), 0, true)]
        );
    }

    #[test]
    fn test_seq_to_string() {
        let char_vec: Vec<char> = to_char_vec("12345");
        let files = diskmap_to_files(char_vec);
        let string = sequence_to_string(&files);
        assert_eq!(string, "0..111....22222");

        let char_vec: Vec<char> = to_char_vec("010204");
        let files = diskmap_to_files(char_vec);
        let string = sequence_to_string(&files);
        assert_eq!(string, ".......");

        let char_vec: Vec<char> = to_char_vec("00011");
        let files = diskmap_to_files(char_vec);
        let string = sequence_to_string(&files);
        assert_eq!(string, ".2");

        let char_vec: Vec<char> = to_char_vec("2333133121414131402");
        let files = diskmap_to_files(char_vec);
        let string = sequence_to_string(&files);
        assert_eq!(string, "00...111...2...333.44.5555.6666.777.888899");
    }
}
