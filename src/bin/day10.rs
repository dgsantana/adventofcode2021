use std::env::args;

use advent::{read_input, timed_run, AdventResult};

const OPEN_BRACKETS: [char; 4] = ['(', '[', '{', '<'];
const CLOSE_BRACKETS: [char; 4] = [')', ']', '}', '>'];
const ERROR_POINTS: [u32; 4] = [3, 57, 1197, 25137];
const FIX_POINTS: [u64; 4] = [1, 2, 3, 4];

#[allow(unused_variables)]
fn parse_part_1(input: &str) -> (u32, u64) {
    let mut stack = vec![];
    let mut error_points = 0;
    let mut fix_points = vec![];

    for (line_no, line) in input.lines().enumerate() {
        stack.clear();
        let mut error = false;
        for (index, bracket) in line.chars().enumerate() {
            if OPEN_BRACKETS.contains(&bracket) {
                stack.push(bracket);
            } else if CLOSE_BRACKETS.contains(&bracket) {
                let pos = CLOSE_BRACKETS.iter().position(|&b| b == bracket).unwrap();
                if !stack.is_empty()
                    && stack
                        .last()
                        .map_or_else(|| false, |&b| b == OPEN_BRACKETS[pos])
                {
                    stack.pop();
                } else if let Some(last_opened) = stack.last() {
                    #[cfg(feature = "info_prints")]
                    {
                        let expected_pos =
                            OPEN_BRACKETS.iter().position(|b| b == last_opened).unwrap();
                        println!(
                            "Mismatch in line {:03}:{:03}: expected {} found {}",
                            line_no + 1,
                            index + 1,
                            &CLOSE_BRACKETS[expected_pos],
                            &bracket
                        );
                    }
                    error = true;
                    error_points += ERROR_POINTS[pos];
                    break;
                }
            }
        }

        if !stack.is_empty() && !error {
            #[cfg(feature = "info_prints")]
            {
                let missing: String = stack
                    .iter()
                    .rev()
                    .map(|o| OPEN_BRACKETS.iter().position(|c| c == o).unwrap())
                    .map(|p| CLOSE_BRACKETS[p])
                    .collect();
                println!("{}|{}", line, missing);
            }
            fix_points.push(
                stack
                    .iter()
                    .rev()
                    .map(|o| OPEN_BRACKETS.iter().position(|c| c == o).unwrap())
                    .map(|p| FIX_POINTS[p])
                    .fold(0, |total, v| total * 5 + v),
            );
        }
    }
    fix_points.sort_unstable();
    let mid = fix_points.len() / 2;
    (error_points, fix_points[mid])
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(10, use_sample)?;
    let points = timed_run!("Part 1", parse_part_1(&input));
    println!("Total points {}", points.0);
    let points = timed_run!("Part 2", parse_part_1(&input));
    println!("Total points {}", points.1);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(10, true).unwrap();
        let points = parse_part_1(&input);
        assert_eq!(points.0, 26397);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(10, true).unwrap();
        let points = parse_part_1(&input);
        assert_eq!(points.1, 288957);
    }
}
