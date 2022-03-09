use day2_dive::dive;
use day2_dive::dive::Command;
use std::{env, error::Error, fs, process, str::FromStr};

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
    let commands = read_commands_from_file(filename)?;
    println!(
        "Horizonal * depth = {}",
        dive::calculate_final_position(&commands)
    );
    println!(
        "Horizonal * depth (with aim) = {}",
        dive::calculate_final_position_with_aim(&commands)
    );
    Ok(())
}

fn read_commands_from_file(filename: &str) -> Result<Vec<Command>, Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;

    // TODO: error handling
    let commands: Vec<Command> = contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| Command::from_str(s).unwrap())
        .collect();

    Ok(commands)
}

fn parse_args(args: &[String]) -> Result<&str, &str> {
    if args.len() < 2 {
        Err("Missing input filename")
    } else {
        Ok(&args[1])
    }
}
