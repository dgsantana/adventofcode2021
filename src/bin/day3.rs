use std::io::{BufRead, BufReader, Read};

use advent::{AdventError, AdventResult};

#[derive(Debug, Default)]
struct Partition<'a> {
    upper: Vec<&'a str>,
    lower: Vec<&'a str>,
}

fn get_gamma_epsilon(input: &[u8]) -> (i32, i32) {
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    let mut result: Vec<(u32, u32)> = Vec::new();

    while let Ok(size) = input.read_line(&mut buffer) {
        if size > 0 {
            let clean_buffer = buffer.trim();

            // Set the correct size based on the first input line.
            if result.is_empty() {
                result.append(&mut [(0, 0)].repeat(clean_buffer.len()));
            }

            for (index, char) in clean_buffer.chars().enumerate() {
                match char {
                    '0' => result[index].0 += 1,
                    '1' => result[index].1 += 1,
                    _ => println!("Invalid data"),
                }
            }
            buffer.clear();
            continue;
        }
        break;
    }

    // Gets the gamma
    let upper = result
        .iter()
        .map(|(u, d)| if u > d { 0 } else { 1 })
        .rev()
        .enumerate()
        .fold(0, |r, (index, b)| r + (2_i32.pow(index as u32) * b));
    // Gets the epsilon
    let lower = result
        .iter()
        .map(|(u, d)| if u < d { 0 } else { 1 })
        .rev()
        .enumerate()
        .fold(0, |r, (index, b)| r + (2_i32.pow(index as u32) * b));
    (upper, lower)
}

fn get_rating(input: &[u8], o2_or_co2: bool) -> AdventResult<i32> {
    let mut input = BufReader::new(input);
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let mut diagnostics: Vec<&str> = buffer.split('\n').collect();

    let code_len = diagnostics.first().ok_or(AdventError::InvalidData)?.len();

    for index in 0..code_len {
        let mut partition = Partition::default();
        for diagnostic in diagnostics {
            match diagnostic.chars().nth(index) {
                Some('1') => partition.upper.push(diagnostic),
                Some('0') => partition.lower.push(diagnostic),
                _ => println!("Invalid data"),
            }
        }
        diagnostics = if o2_or_co2 {
            if partition.upper.len() >= partition.lower.len() {
                partition.upper
            } else {
                partition.lower
            }
        } else if partition.upper.len() < partition.lower.len() {
            partition.upper
        } else {
            partition.lower
        };
        if diagnostics.len() == 1 {
            break;
        }
    }

    // Gets the gamma
    let o2_rating = i32::from_str_radix(diagnostics[0], 2)?;
    Ok(o2_rating)
}

fn main() -> AdventResult<()> {
    let input = include_bytes!("../../day3.txt");
    let (gamma, epsilon) = get_gamma_epsilon(input);
    println!(
        "Gamma is {} and epsilon is {} and power consumption is {}",
        gamma,
        epsilon,
        gamma * epsilon
    );
    let o2 = get_rating(input, true)?;
    let co2 = get_rating(input, false)?;
    println!(
        "oxygen generator rating is {} and CO2 scrubber rating is {} and life support rating is {}",
        o2,
        co2,
        o2 * co2
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{get_gamma_epsilon, get_rating};
    const INPUT: &[u8; 71] =
        b"00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010";

    #[test]
    fn validate_gamma_epsilon() {
        let (gamma, epsilon) = get_gamma_epsilon(INPUT);
        assert_eq!(gamma, 22);
        assert_eq!(epsilon, 9);
    }

    #[test]
    fn validate_o2_rating() {
        let o2_rating = get_rating(INPUT, true).expect("Failed to parse data.");
        assert_eq!(o2_rating, 23);
    }

    #[test]
    fn validate_co2_rating() {
        let co2_rating = get_rating(INPUT, false).expect("Failed to parse data.");
        assert_eq!(co2_rating, 10);
    }
}
