use std::io::{BufRead, BufReader};

use advent::{AdventError, AdventResult};

#[derive(Debug)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
    Unknown,
}

#[derive(Debug, Default)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

impl Position {
    fn factor(&self) -> i32 {
        self.horizontal * self.depth
    }
}

fn command_parser(input: &[u8]) -> AdventResult<Vec<Command>> {
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    let mut commands = vec![];

    while let Ok(size) = input.read_line(&mut buffer) {
        if size > 0 {
            let clean_buffer = buffer.trim();
            let parts: Vec<_> = clean_buffer.split(' ').collect();
            if parts.len() != 2 {
                return Err(AdventError::InvalidData);
            }
            let amount = parts[1].parse::<i32>()?;
            let command = match parts[0] {
                "forward" => Command::Forward(amount),
                "down" => Command::Down(amount),
                "up" => Command::Up(amount),
                _ => Command::Unknown,
            };
            commands.push(command);
            buffer.clear();
            continue;
        }
        break;
    }
    Ok(commands)
}

fn determine_position(commands: &[Command]) -> Position {
    let mut pos = Position::default();
    for command in commands {
        match command {
            Command::Up(amount) => pos.aim -= amount,
            Command::Down(amount) => pos.aim += amount,
            Command::Forward(amount) => {
                pos.depth += pos.aim * amount;
                pos.horizontal += amount;
            }
            Command::Unknown => println!("Invalid command"),
        }
    }
    pos
}

fn main() -> AdventResult<()> {
    let input = include_bytes!("../../day2.txt");
    let commands = command_parser(input)?;
    let pos = determine_position(&commands);
    println!(
        "Horizontal position is {} and depth is {} with a factor of {}",
        pos.horizontal,
        pos.depth,
        pos.factor()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{command_parser, determine_position};

    #[test]
    fn validate() {
        let input = b"forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2";
        let commands = command_parser(input).expect("invalid input data.");
        let pos = determine_position(&commands);
        assert_eq!(pos.horizontal, 15);
        assert_eq!(pos.depth, 60);
        assert_eq!(pos.factor(), 900);
    }
}
