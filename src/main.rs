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
    let mut times = vec![0];
    let t0 = Instant::now();

    println!("Day 1:");
    run_command("day-01", "part1");
    run_command("day-01", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 2:");
    run_command("day-02", "part1");
    run_command("day-02", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 3:");
    run_command("day-03", "part1");
    run_command("day-04", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 4:");
    run_command("day-04", "part1");
    run_command("day-04", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 5:");
    run_command("day-05", "part1");
    run_command("day-05", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 6:");
    run_command("day-06", "part1");
    run_command("day-06", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 7:");
    run_command("day-07", "part1");
    run_command("day-07", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 8:");
    run_command("day-08", "part1");
    run_command("day-08", "part2");

    times.push(t0.elapsed().as_millis());

    println!("Day 9:");
    run_command("day-09", "part1");
    run_command("day-09", "part2");

    times.push(t0.elapsed().as_millis());

    println!("-----------------------------");
    let new_times: Vec<_> = times
        .windows(2)
        .map(|window| {
            let (x, y) = (window[0], window[1]);
            (x as i128 - y as i128).abs() as u128
        })
        .collect();

    println!("{:<10} {:<15}", "Day", "Time Elapsed (ms)");
    println!("-----------------------------");
    for (day, time) in new_times.iter().enumerate() {
        println!("{:<10} {:<15}", day + 1, time);
    }
}
