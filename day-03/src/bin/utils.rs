use std::fs;

const INPUT_DIR: &str = "../inputs";
pub fn read_input_to_string(day: u8, test: Option<bool>) -> String {
    let test = test.unwrap_or(false);
    let ext = if test { "t" } else { "" };
    let filename = format!("{}/day{:02}{}.txt", INPUT_DIR, day, ext);
    fs::read_to_string(filename).expect("Failed to read input file")
}
