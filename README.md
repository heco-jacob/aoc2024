## Advent of Code 2024

## When a new day dawns...

* Run: 
    ```bash
    cargo new day-XX 
    ```
* Add today's day to `src/main.rs`
* Add today's day to `Cargo.toml`
* Add following to the new day's `Cargo.toml` to correctly reference common utils:
    ```
    [[bin]]
    name = "part1"
    path = "src/bin/part1.rs"
    
    [[bin]]
    name = "part2"
    path = "src/bin/part2.rs"
    
    [dependencies]
    utils = { path = "../utils" }
    ```
* Create binary folder `bin` and place `part1.rs` and `part2.rs` (and delete `main.rs`)



## How to run 

Run all days: 
```
cargo run 
```

Navigate to day-XX and run

```angular2html
cargo run --bin part1 
```
or 
```angular2html
cargo run --bin part2  
```
Possible flags
```angular2html
-t for running test data
-m for measure time of function 
```


### Rust 101

- cargo new <project name> 
- cargo run --bin part1 to run specific bin
- cargo fmt to format code
- cargo test part1 to run part 1 tests

## Structure

- New cargo for each day
- Separate each part in different bins

## Days:

#### --- Day 1: Historian Hysteria ---

```
3   4
4   3
2   5
1   3
3   9
3   3
```

#### --- Day 2: Red-Nosed Reports ---

```
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
```

#### --- Day 3: Mull It Over ---

``` 
xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
```

#### --- Day 4: Ceres Search ---

```angular2html
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
```
#### --- Day 5: Print Queue ---
 
```angular2html
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
```

#### --- Day 6: Guard Gallivant ---

```
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
```

#### --- Day 7: Bridge Repair ---

```angular2html
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
```

#### --- Day 8: Resonant Collinearity ---

```
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
```