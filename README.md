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
