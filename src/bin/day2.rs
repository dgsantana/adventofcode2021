use std::env::args;

use advent::{read_input, AdventError, AdventResult, timed_run};

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

fn parse_input(input: &str) -> AdventResult<Vec<Command>> {
    let mut commands = vec![];

    for line in input.lines() {
        let clean_buffer = line.trim();
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
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(2, use_sample)?;
    let commands = parse_input(&input)?;
    let pos = timed_run!("Part 1 and 2", determine_position(&commands));
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
    use super::*;

    #[test]
    fn validate() {
        let input = read_input(2, true).expect("Invalid data");
        let commands = parse_input(&input).expect("Invalid data");
        let pos = determine_position(&commands);
        assert_eq!(pos.horizontal, 15);
        assert_eq!(pos.depth, 60);
        assert_eq!(pos.factor(), 900);
    }
}
