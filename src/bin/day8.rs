use std::env::args;

use advent::{read_input, timed_run, AdventError, AdventResult};

const NUMBERS: [&str; 10] = [
    "abcefg", "cf", "acdeg", "acdfg", "bcdf", "abdfg", "abdefg", "acf", "abcdefg", "abcdfg",
];

#[derive(Debug, Default)]
struct SegmentDisplay {
    input: Vec<Vec<char>>,
    output: Vec<Vec<char>>,
}

impl SegmentDisplay {
    fn new(input_numbers: Vec<Vec<char>>, output_numbers: Vec<Vec<char>>) -> Self {
        Self {
            input: input_numbers,
            output: output_numbers,
        }
    }

    fn value(&self) -> u64 {
        let mut result = 0;
        let size = self.output.len() as u32 - 1;
        for (index, digit) in self.output.iter().enumerate() {
            if let Some(n) = self.digit(digit) {
                result += n as u64 * 10_u64.pow(size - index as u32);
            }
        }
        result
    }

    fn all_digits(&self) -> impl Iterator<Item = &Vec<char>> {
        self.input.iter().chain(self.output.iter())
    }

    fn digit_1(&self) -> Vec<char> {
        self.all_digits()
            .find(|x| x.len() == 2)
            .map_or(vec![], |x| x.clone())
    }

    fn digit_4(&self) -> Vec<char> {
        self.all_digits()
            .find(|x| x.len() == 4)
            .map_or(vec![], |x| x.clone())
    }

    fn digit(&self, digit: &[char]) -> Option<u32> {
        match digit.len() {
            2 => Some(1),
            3 => Some(7),
            4 => Some(4),
            5 => {
                // Can be 2, 3 or 5
                if digit_includes(digit, &self.digit_1()) {
                    Some(3)
                } else {
                    let digit_bd = digit_xor(&self.digit_4(), &self.digit_1());
                    if digit_includes(digit, &digit_bd) {
                        Some(5)
                    } else {
                        Some(2)
                    }
                }
            }
            6 => {
                // Can be 0, 6 or 9
                if digit_includes(digit, &self.digit_1()) {
                    if digit_includes(digit, &self.digit_4()) {
                        Some(9)
                    } else {
                        Some(0)
                    }
                } else {
                    Some(6)
                }
            }
            7 => Some(8),
            _ => None,
        }
    }
}

fn digit_includes(digit: &[char], base: &[char]) -> bool {
    base.iter().all(|&x| digit.contains(&x))
}

fn digit_xor(digit: &[char], base: &[char]) -> Vec<char> {
    digit
        .iter()
        .filter(|&x| !base.contains(x))
        .cloned()
        .collect()
}

fn parse_input(input: &str) -> AdventResult<Vec<SegmentDisplay>> {
    let mut result = vec![];
    for line in input.lines() {
        let (input_part, output_part) = line.split_once('|').ok_or(AdventError::InvalidData)?;
        let input_numbers = input_part
            .split_whitespace()
            .map(|n| n.chars().collect())
            .collect();

        let output_numbers = output_part
            .split_whitespace()
            .map(|n| n.chars().collect())
            .collect();
        result.push(SegmentDisplay::new(input_numbers, output_numbers));
    }
    Ok(result)
}

fn solve_part1(segment_display: &mut [SegmentDisplay]) -> AdventResult<usize> {
    let search_pattern = [
        NUMBERS[1].len(),
        NUMBERS[4].len(),
        NUMBERS[7].len(),
        NUMBERS[8].len(),
    ];
    let mut count = 0;
    for segment_number in segment_display.iter() {
        count += segment_number
            .output
            .iter()
            .filter(|x| search_pattern.contains(&x.len()))
            .count();
    }
    Ok(count)
}

fn solve_part2(segment_display: &mut [SegmentDisplay]) -> u64 {
    segment_display.iter().map(|s| s.value()).sum()
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(8, use_sample)?;
    let mut numbers = parse_input(&input)?;
    let count = timed_run!("Part 1", solve_part1(&mut numbers))?;
    if use_sample {
        // dbg!(&numbers);
    }
    println!("Total number of 1, 4, 7 or 8: {}", count);

    let count = timed_run!("Part 2", solve_part2(&mut numbers));
    println!("Total is {}", count);
    if use_sample {
        // dbg!(numbers);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(8, true).unwrap();
        let mut numbers = parse_input(&input).unwrap();
        let count = solve_part1(&mut numbers).unwrap();
        assert_eq!(count, 26);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(8, true).unwrap();
        let mut numbers = parse_input(&input).unwrap();
        let count = solve_part2(&mut numbers);
        assert_eq!(count, 61229);
    }
}
