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

    let mut search_strings: Vec<String> = Vec::new();

    // horizontal
    for vec in &xmas_array {
        search_strings.push(vec.iter().collect());
    }

    // vertical
    for c in 0..xmas_array[0].len() {
        let mut horizontal = Vec::new();
        for r in 0..xmas_array.len() {
            horizontal.push(xmas_array[r][c]);
        }
        search_strings.push(horizontal.iter().collect());
    }

    // diagonal
    for c in 0..xmas_array[0].len() {
        let mut diagonal = Vec::new();
        for i in 0..xmas_array.len() - c {
            diagonal.push(xmas_array[i][c + i]);
        }
    }

    let rows = xmas_array.len();
    let cols = xmas_array[0].len();
    let mut diagonals: Vec<String> = Vec::new();

    // \-diagonals
    for start_col in 0..cols {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = start_col;
        while row < rows && col < cols {
            diagonal.push(xmas_array[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal.iter().collect());
    }

    for start_row in 1..rows {
        // first row already covered ^
        let mut diagonal = Vec::new();
        let mut row = start_row;
        let mut col = 0;
        while row < rows && col < cols {
            diagonal.push(xmas_array[row][col]);
            row += 1;
            col += 1;
        }
        diagonals.push(diagonal.iter().collect());
    }

    // /-diagonals
    for start_col in (0..cols).rev() {
        let mut diagonal = Vec::new();
        let mut row = 0;
        let mut col = start_col;
        while row < rows && col < cols {
            diagonal.push(xmas_array[row][col]);
            row += 1;
            if col > 0 {
                col -= 1;
            } else {
                break;
            }
        }
        diagonals.push(diagonal.iter().collect());
    }

    for start_row in 1..rows {
        // first row already covered ^
        let mut diagonal = Vec::new();
        let mut row = start_row;
        let mut col = cols - 1;
        while row < rows && col < cols {
            diagonal.push(xmas_array[row][col]);
            row += 1;
            if col > 0 {
                col -= 1;
            } else {
                break;
            }
        }
        diagonals.push(diagonal.iter().collect());
    }
    for diag in diagonals {
        search_strings.push(diag);
    }

    for ss in search_strings {
        let sss = ss.chars().collect::<Vec<char>>();
        let a = sss
            .windows(4)
            .filter(|x| *x == ['X', 'M', 'A', 'S'] || *x == ['S', 'A', 'M', 'X'])
            .count();
        score += a as i32;
    }

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
