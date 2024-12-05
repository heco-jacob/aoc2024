mod utils;

use crate::utils::read_input_to_string;

fn main() {
    let mut score: i32 = 0;
    let input = read_input_to_string(4, Some(false));
    let mut xmas_array: Vec<Vec<char>> = Vec::new();

    for i in input.lines() {
        let mut vec_line = Vec::new();
        for c in i.chars() {
            vec_line.push(c);
        }
        xmas_array.push(vec_line);
    }

    for c in 1..xmas_array[0].len() - 1 {
        for r in 1..xmas_array.len() - 1 {
            let char = xmas_array[c][r];
            if char == 'A' {
                // up left r=-1 c=-1
                // up right r=-1 c=+1
                // down left r=+1 c=-1
                // down right r=+1 c=+1
                let prim = format!(
                    "{}{}{}",
                    xmas_array[c - 1][r - 1],
                    char,
                    xmas_array[c + 1][r + 1]
                );
                let sec = format!(
                    "{}{}{}",
                    xmas_array[c - 1][r + 1],
                    char,
                    xmas_array[c + 1][r - 1]
                );
                println!("prim:{}", prim);
                println!("sec:{}", sec);
                println!("col:{}, row: {}", c, r);
                if (prim == "MAS" || prim == "SAM") && (sec == "MAS" || sec == "SAM") {
                    score += 1;
                }
            }
        }
    }

    // for ss in search_strings{
    //     let sss = ss.chars().collect::<Vec<char>>();
    //     let a = sss.windows(4).filter(|x| *x==['X', 'M', 'A', 'S'] || *x==['S','A','M','X']).count();
    //     score+=a as i32;
    // }

    println!("{}", score);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let input = read_input_to_string(4, Some(true));
        let mut xmas_array: Vec<Vec<char>> = Vec::new();

        for i in input.lines() {
            let mut vec_line = Vec::new();
            for c in i.chars() {
                vec_line.push(c);
            }
            xmas_array.push(vec_line);
        }
        // MMMSXXMASM
        // MSAMXMSMSA
        // AMXSXMAAMM
        // MSAMASMSMX
        // XMASAMXAMM
        // XXAMMXXAMA
        // SMSMSASXSS
        // SAXAMASAAA
        // MAMMMXMMMM
        // MXMXAXMASX
        assert_eq!(xmas_array[0][0], 'M');
        assert_eq!(xmas_array[0][1], 'M');
        assert_eq!(xmas_array[0][2], 'M');
        assert_eq!(xmas_array[0][3], 'S');
        assert_eq!(xmas_array[0][4], 'X');
        assert_eq!(xmas_array[0][5], 'X');

        assert_eq!(xmas_array[1][1], 'S');

        assert_eq!(xmas_array[9][9], 'X');
    }
}
