use std::env;
use std::process::{Command, Stdio};
use std::time::Instant;

fn run_command(package: &str, binary: &str) {
    let test_flag = env::args()
        .find(|arg| arg == "-t")
        .unwrap_or_else(|| "".to_string());
    let command = format!(
        "{} {} {} {} {} {} {} {}",
        "run", "--bin", binary, "-p", package, "-q", "--", test_flag
    );
    // println!("Command: {}", command);

    // TODO: some GPT help here, read later
    let output = Command::new("cargo")
        .args(&[
            "run", "--bin", binary, "-p", package, "-q", "--", &test_flag,
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .output()
        .expect(&format!("Failed to run {} - {}", package, binary));

    if !output.stdout.is_empty() {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }

    if !output.status.success() {
        eprintln!("Error running {} - {}", package, binary);
    }
}

fn main() {
    let measure_flag = env::args().any(|arg| arg == "-measure" || arg == "-m");

    let t1 = Instant::now();
    println!("Day 1:");
    run_command("day-01", "part1");
    run_command("day-01", "part2");

    let t2 = Instant::now();
    if measure_flag {
        println!("Time elapsed: {:?}", t1.duration_since(t2));
    }

    println!("Day 2:");
    run_command("day-02", "part1");
    run_command("day-02", "part2");

    let t3 = Instant::now();
    if measure_flag {
        println!("Time elapsed: {:?}", t2.duration_since(t3));
    }

    println!("Day 3:");
    run_command("day-03", "part1");
    run_command("day-04", "part2");

    let t4 = Instant::now();
    if measure_flag {
        println!("Time elapsed: {:?}", t3.duration_since(t4));
    }

    println!("Day 4:");
    run_command("day-04", "part1");
    run_command("day-04", "part2");

    let t5 = Instant::now();
    if measure_flag {
        println!("Time elapsed: {:?}", t4.duration_since(t5));
    }

    println!("Day 5:");
    run_command("day-05", "part1");
    run_command("day-05", "part2");

    let t6 = Instant::now();
    if measure_flag {
        println!("Time elapsed: {:?}", t5.duration_since(t6));
    }

    println!("Day 6:");
    run_command("day-06", "part1");
    run_command("day-06", "part2");

    let t7 = Instant::now();
    if measure_flag {
        println!("Time elapsed: {:?}", t6.duration_since(t7));
    }
}
