pub mod dive {
    use std::{cmp::max, str::FromStr, string::ParseError};

    use strum_macros::EnumString;

    #[derive(Debug, EnumString, PartialEq)]
    pub enum Direction {
        #[strum(ascii_case_insensitive)]
        FORWARD,
        #[strum(ascii_case_insensitive)]
        DOWN,
        #[strum(ascii_case_insensitive)]
        UP,
    }

    #[derive(Debug, PartialEq)]
    pub struct Command {
        pub direction: Direction,
        pub units: i32,
    }

    impl Command {
        pub fn get_signed_units(&self) -> i32 {
            match self.direction {
                Direction::UP => -self.units,
                _ => self.units,
            }
        }
    }

    impl FromStr for Command {
        type Err = ParseError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let tokens: Vec<&str> = s.split_whitespace().collect();

            // TODO: add better error checking
            let direction_str = tokens.get(0).unwrap_or(&"");
            let units = tokens.get(1).unwrap_or(&"0").parse::<i32>().unwrap();

            Ok(Command {
                direction: Direction::from_str(direction_str).unwrap(),
                units,
            })
        }
    }

    fn print_final_position(horiz: i32, depth: i32) {
        println!("Final position: horizontal {}, depth {}", horiz, depth);
    }

    pub fn calculate_final_position(commands: &Vec<Command>) -> i64 {
        let mut horizontal_position = 0;
        let mut depth_position = 0;

        for command in commands {
            if command.direction == Direction::FORWARD {
                horizontal_position += command.get_signed_units();
            } else {
                depth_position = max(0, depth_position + command.get_signed_units());
            }
        }

        print_final_position(horizontal_position, depth_position);
        horizontal_position as i64 * depth_position as i64
    }

    pub fn calculate_final_position_with_aim(commands: &Vec<Command>) -> i64 {
        let mut horizontal_position = 0;
        let mut depth_position = 0;
        let mut aim = 0;

        for command in commands {
            if command.direction == Direction::FORWARD {
                horizontal_position += command.get_signed_units();
                depth_position += command.get_signed_units() * aim;
            } else {
                aim = max(0, aim + command.get_signed_units());
            }
        }

        print_final_position(horizontal_position, depth_position);
        horizontal_position as i64 * depth_position as i64
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dive::{Command, Direction};
    use parameterized::{ide, parameterized};
    use std::str::FromStr;

    ide!();

    #[parameterized(command = {"up", "down", "forward"})]
    fn test_parse_valid_command(command: &str) {
        assert_eq!(
            Command {
                direction: Direction::from_str(command).unwrap(),
                units: 5
            },
            dive::Command::from_str(&format!("{} {}", command, 5)).unwrap()
        );
    }

    #[test]
    fn test_parse_invalid_command() {
        assert!(dive::Command::from_str("downf 5").is_err());
    }

    #[test]
    fn test_parse_invalid_unit() {
        assert!(dive::Command::from_str("down f").is_err());
    }

    #[test]
    fn test_calculate_final_position_with_aim() {
        let command_strs = vec![
            "forward 5",
            "down 5",
            "forward 8",
            "up 3",
            "down 8",
            "forward 2",
        ];
        let commands = command_strs
            .iter()
            .map(|s| Command::from_str(s).unwrap())
            .collect();
        assert_eq!(900, dive::calculate_final_position_with_aim(&commands));
    }
}
