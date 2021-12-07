use std::{
    env::args,
    io::{BufRead, BufReader},
};

use advent::{AdventError, AdventResult};

const SAMPLE: &str = "16,1,2,0,4,2,7,1,2,14";

fn parse(input: &[u8]) -> Vec<u64> {
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    let mut result = Vec::new();
    while let Ok(size) = reader.read_line(&mut buffer) {
        if size == 0 {
            break;
        }
        let clean_buffer = buffer.trim();
        result.append(
            &mut clean_buffer
                .split(',')
                .filter_map(|n| n.trim().parse::<u64>().ok())
                .collect(),
        );
        buffer.clear();
    }
    result
}

/// Brute force approach
fn calculate_fuel_part1_lazy(positions: &[u64]) -> AdventResult<(u64, u64)> {
    let min = *positions.iter().min().ok_or(AdventError::InvalidData)?;
    let max = *positions.iter().max().ok_or(AdventError::InvalidData)?;
    let mut fuel_cost = u64::MAX;
    let mut best_position = 0;
    for pos in min..=max {
        let fuel: u64 = positions
            .iter()
            .map(|&v| if v > pos { v - pos } else { pos - v })
            .sum();
        if fuel < fuel_cost {
            fuel_cost = fuel;
            best_position = pos;
        }
    }
    Ok((fuel_cost, best_position))
}

/// A much smarter approach is to use the median as the best position.
fn calculate_fuel_part1_smart(positions: &[u64]) -> AdventResult<(u64, u64)> {
    let mut positions = positions.to_vec();
    positions.sort_unstable();
    let median = positions[positions.len() / 2];
    let best_position = median;
    let fuel_cost = positions
        .iter()
        .map(|&v| {
            if v > best_position {
                v - best_position
            } else {
                best_position - v
            }
        })
        .sum();
    Ok((fuel_cost, best_position))
}

/// Brute force approach
fn calculate_fuel_part2(positions: &[u64]) -> AdventResult<(u64, u64)> {
    let min = *positions.iter().min().ok_or(AdventError::InvalidData)?;
    let max = *positions.iter().max().ok_or(AdventError::InvalidData)?;
    let mut fuel_cost = u64::MAX;
    let mut best_position = 0;
    for pos in min..=max {
        let fuel: u64 = positions
            .iter()
            .map(|&v| if v > pos { v - pos } else { pos - v })
            .map(|v| (1..=v).sum::<u64>())
            .sum();
        if fuel < fuel_cost {
            fuel_cost = fuel;
            best_position = pos;
        }
    }
    Ok((fuel_cost, best_position))
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let slow = args().any(|arg| arg == "--slow");
    let input = if use_sample {
        SAMPLE.as_bytes()
    } else {
        include_bytes!("../../day7.txt")
    };
    let positions = parse(input);
    let (fuel_cost, best_position) = if slow {
        calculate_fuel_part1_lazy(&positions)?
    } else {
        calculate_fuel_part1_smart(&positions)?
    };
    println!("Part 1");
    println!(
        "Total fuel cost is {} at position {}",
        fuel_cost, best_position
    );
    println!();
    println!("Part 2");
    let (fuel_cost, best_position) = calculate_fuel_part2(&positions)?;
    println!(
        "Total fuel cost is {} at position {}",
        fuel_cost, best_position
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{
        calculate_fuel_part1_lazy, calculate_fuel_part1_smart, calculate_fuel_part2, parse, SAMPLE,
    };

    #[test]
    fn validate_fuel_part1_lazy() {
        let positions = parse(SAMPLE.as_bytes());
        let result = calculate_fuel_part1_lazy(&positions).expect("Invalid data");
        assert_eq!(result, (37, 2));
    }

    #[test]
    fn validate_fuel_part1_smart() {
        let positions = parse(SAMPLE.as_bytes());
        let result = calculate_fuel_part1_smart(&positions).expect("Invalid data");
        assert_eq!(result, (37, 2));
    }

    #[test]
    fn validate_fuel_part2_lazy() {
        let positions = parse(SAMPLE.as_bytes());
        let result = calculate_fuel_part2(&positions).expect("Invalid data");
        assert_eq!(result, (168, 5));
    }
}
