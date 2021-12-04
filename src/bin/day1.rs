use std::io::{BufRead, BufReader};

use advent::AdventResult;

fn input_parse(input: &[u8]) -> AdventResult<Vec<i32>> {
    let mut reader = BufReader::new(input);
    let mut buf = String::new();
    let mut input = vec![];
    while let Ok(size) = reader.read_line(&mut buf) {
        if size > 0 {
            let buf = buf.trim();
            if buf.is_empty() {
                continue;
            }
            input.push(buf.parse::<i32>()?);
        } else {
            break;
        }
        buf.clear();
    }
    Ok(input)
}

fn part_one(measurements: &[i32]) -> i32 {
    let mut last = measurements[0];
    let mut counter = 0;
    for current in measurements.iter().skip(1) {
        if *current > last {
            counter += 1;
        }
        last = *current;
    }
    println!(
        "There are {} measurements that are larger then the previous measurement on a total of {}.",
        counter,
        measurements.len()
    );
    counter
}

fn part_two(measurements: &[i32]) -> i32 {
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
    println!(
        "There are {} triplets measurements that are larger then the previous measurement on a total of {}.",
        counter,
        measurements.len()
    );

    counter
}

fn main() -> AdventResult<()> {
    println!("Advent of Code 2021! Rust edition. Day 1");
    let input = include_bytes!("../../day1.txt");
    let measurements = input_parse(input)?;
    part_one(&measurements);
    part_two(&measurements);
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{input_parse, part_one, part_two};

    #[test]
    fn validate_part_one() {
        let input = b"199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        let measurements = input_parse(input).expect("invalid data");
        let counter = part_one(&measurements);
        assert_eq!(counter, 7);
    }

    #[test]
    fn validate_part_two() {
        let input = b"199\n200\n208\n210\n200\n207\n240\n269\n260\n263";
        let measurements = input_parse(input).expect("invalid data");
        let counter = part_two(&measurements);
        assert_eq!(counter, 5);
    }
}
