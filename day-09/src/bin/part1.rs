use utils::read_input_to_string;

fn pop_last_number(vec: &mut Vec<Option<u32>>) -> Option<u32> {
    while let Some(last) = vec.pop() {
        if let Some(num) = last {
            return Some(num);
        }
    }
    None
}

fn sort_and_slice(sequence: Vec<Option<u32>>) -> Vec<Option<u32>> {
    let mut seq = sequence.clone();
    let mut idx = 0;

    while idx < seq.len() {
        if seq[idx].is_none() {
            // Pop a number from the end and replace the `None`
            if let Some(value) = pop_last_number(&mut seq) {
                seq[idx] = Some(value);
            } else {
                // No more numbers to pop, break early
                break;
            }
        }
        idx += 1;
    }

    seq
}
fn main() {
    let mut score: i64 = 0;
    let input: String = read_input_to_string(9);

    // 2333133121414131402
    // 00...111...2...333.44.5555.6666.777.888899

    let charvec: Vec<char> = input.chars().collect();
    let mut sequence: Vec<Option<u32>> = Vec::new();
    let mut num: u32 = 0;
    println!("{}", input);

    for (idx, c) in charvec.iter().enumerate() {
        if let Some(digit) = c.to_digit(10) {
            if (idx % 2 == 0) {
                for _ in 0..digit {
                    sequence.push(Some(num));
                }
            } else {
                for _ in 0..digit {
                    sequence.push(None);
                }
                num += 1;
            }
        }
    }
    println!("Sequence {:?}", sequence);

    let new = sort_and_slice(sequence.clone());

    for (idx, id) in new.iter().enumerate() {
        if let Some(value) = id {
            score += (idx as u32 * value) as i64;
        }
    }

    println!("SEQ: {:?}", new);
    println!("{}", score);
}
