use day1_sonar_sweep;
use std::{env, error::Error, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = parse_args(&args).unwrap_or_else(|err| {
        println!("Failed to parse arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = run(filename) {
        println!("Error: {}", e);
        process::exit(1);
    };
}

fn run(filename: &str) -> Result<(), Box<dyn Error>> {
    let numbers = read_measurements_from_file(filename)?;
    let count = day1_sonar_sweep::sonar_sweep::count_increase(&numbers);
    println!(
        "{} measurements are larger than the previous measurement!",
        count
    );

    let group_count = day1_sonar_sweep::sonar_sweep::count_group_increase(&numbers, 3);
    println!(
        "{} measurement groups are larger than the previous measurement group!",
        group_count
    );

    Ok(())
}

fn read_measurements_from_file(filename: &str) -> Result<Vec<i32>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    let numbers: Vec<i32> = contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect();

    Ok(numbers)
}

fn parse_args(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        Err("Missing input filename")
    } else {
        Ok(&args[1])
    }
}
