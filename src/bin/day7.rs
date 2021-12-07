use std::env::args;

use advent::{read_input, timed_run, AdventError, AdventResult};

fn parse_input(input: &str) -> Vec<u64> {
    let mut result = Vec::new();
    for line in input.lines() {
        let clean_buffer = line.trim();
        result.append(
            &mut clean_buffer
                .split(',')
                .filter_map(|n| n.trim().parse::<u64>().ok())
                .collect(),
        );
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
    let input = read_input(7, use_sample)?;
    let positions = parse_input(&input);
    let (fuel_cost, best_position) = if slow {
        timed_run!("Part 1", calculate_fuel_part1_lazy(&positions))?
    } else {
        timed_run!("Part 1", calculate_fuel_part1_smart(&positions))?
    };
    println!(
        "Total fuel cost is {} at position {}",
        fuel_cost, best_position
    );
    println!();
    let (fuel_cost, best_position) = timed_run!("Part 2", calculate_fuel_part2(&positions))?;
    println!(
        "Total fuel cost is {} at position {}",
        fuel_cost, best_position
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_fuel_part1_lazy() {
        let input = read_input(7, true).expect("Invalid data");
        let positions = parse_input(&input);    
        let result = calculate_fuel_part1_lazy(&positions).expect("Invalid data");
        assert_eq!(result, (37, 2));
    }

    #[test]
    fn validate_fuel_part1_smart() {
        let input = read_input(7, true).expect("Invalid data");
        let positions = parse_input(&input);    
        let result = calculate_fuel_part1_smart(&positions).expect("Invalid data");
        assert_eq!(result, (37, 2));
    }

    #[test]
    fn validate_fuel_part2_lazy() {
        let input = read_input(7, true).expect("Invalid data");
        let positions = parse_input(&input);    
        let result = calculate_fuel_part2(&positions).expect("Invalid data");
        assert_eq!(result, (168, 5));
    }
}
