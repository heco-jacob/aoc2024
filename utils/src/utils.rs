use std::env;
use std::fs;
use std::path::PathBuf;

pub fn read_input_to_string(day: u8) -> String {
    let test = env::args().any(|arg| arg == "-t");
    let ext = if test { "t" } else { "" };
    let filename = format!("day{:02}{}.txt", day, ext);

    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // move up one level to directory of workspace cargo (root)
    path.push("inputs");
    path.push(filename);

    fs::read_to_string(&path).expect(&format!("Failed to read input file: {:?}", path))
}
