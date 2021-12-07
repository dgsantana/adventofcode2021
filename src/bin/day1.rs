use std::env::args;

use advent::{read_input, timed_run, AdventResult};

fn input_parse(input: &str) -> AdventResult<Vec<i32>> {
    let mut result = vec![];
    for line in input.lines() {
        let buf = line.trim();
        if buf.is_empty() {
            continue;
        }
        result.push(buf.parse::<i32>()?);
    }
    Ok(result)
}

fn part_one(measurements: &[i32]) -> (i32, usize) {
    let mut last = measurements[0];
    let mut counter = 0;
    for current in measurements.iter().skip(1) {
        if *current > last {
            counter += 1;
        }
        last = *current;
    }
    (counter, measurements.len())
}

fn part_two(measurements: &[i32]) -> (i32, usize) {
    let mut last = measurements.iter().take(3).sum();
    let mut counter = 0;
    for current in measurements
        .iter()
        .skip(1)
        .zip(measurements.iter().skip(2))
        .zip(measurements.iter().skip(3))
        .map(|f| f.0 .0 + f.0 .1 + f.1)
    {
        if current > last {
            counter += 1;
        }
        last = current;
    }
    (counter, measurements.len())
}

fn main() -> AdventResult<()> {
    println!("Advent of Code 2021! Rust edition. Day 1");
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(1, use_sample)?;
    let data = input_parse(&input)?;
    let (count, amount) = timed_run!("Part 1", part_one(&data));
    println!(
        "There are {} measurements that are larger then the previous measurement on a total of {}.",
        count, amount
    );

    let (count, amount) = timed_run!("Part 2", part_two(&data));
    println!(
        "There are {} triplets measurements that are larger then the previous measurement on a total of {}.",
        count,
        amount
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn validate_part_one() {
        let input = read_input(1, true).expect("Invalid data");
        let measurements = input_parse(&input).expect("invalid data");
        let (counter, _) = part_one(&measurements);
        assert_eq!(counter, 7);
    }

    #[test]
    fn validate_part_two() {
        let input = read_input(1, true).expect("Invalid data");
        let measurements = input_parse(&input).expect("invalid data");
        let (counter, _) = part_two(&measurements);
        assert_eq!(counter, 5);
    }
}
