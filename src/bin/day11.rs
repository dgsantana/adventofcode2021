use std::env::args;

use advent::{read_input, timed_run, AdventResult};

#[derive(Debug, Default)]
struct Grid {
    width: isize,
    height: isize,
    data: Vec<u8>,
}

impl Grid {
    fn new(width: isize, height: isize, data: Vec<u8>) -> Self {
        Self {
            width,
            height,
            data,
        }
    }

    #[allow(unused_variables)]
    fn part_1(&mut self, steps: usize) -> usize {
        let mut counter = 0;
        for step in 0..steps {
            #[cfg(feature = "info_prints")]
            println!("Step {}", step + 1);
            let mut flashed = vec![];
            for y in 0..self.height {
                for x in 0..self.width {
                    self.iterate_octopus(x, y, &mut flashed);
                }
            }

            counter += flashed.len();
            #[cfg(feature = "info_prints")]
            {
                for (y, line) in self.data.chunks(self.width as usize).enumerate() {
                    for (x, byte) in line.iter().enumerate() {
                        if flashed.contains(&(x as isize, y as isize)) {
                            print!("{}", byte);
                        } else {
                            print!("\x1B[38;5;8m{}\x1B[0m", byte);
                        }
                    }
                    println!();
                }
                println!();
            }
        }
        counter
    }

    /// Loop until all the octopus flash
    /// `steps` is used for debugging and requires feature `info_prints` to show something
    #[allow(unused_variables)]
    fn part_2(&mut self, steps: &[usize]) -> usize {
        let mut step = 0;
        loop {
            step += 1;
            #[cfg(feature = "info_prints")]
            println!("Step {}", step + 1);

            let mut flashed = vec![];
            for y in 0..self.height {
                for x in 0..self.width {
                    self.iterate_octopus(x, y, &mut flashed);
                }
            }
            
            #[cfg(feature = "info_prints")]
            if steps.contains(&step) {
                for (y, line) in self.data.chunks(self.width as usize).enumerate() {
                    for (x, byte) in line.iter().enumerate() {
                        if flashed.contains(&(x as isize, y as isize)) {
                            print!("{}", byte);
                        } else {
                            print!("\x1B[38;5;8m{}\x1B[0m", byte);
                        }
                    }
                    println!();
                }
                println!();
            }

            if flashed.len() == self.data.len() {
                break;
            }
        }
        step
    }

    fn iterate_octopus(&mut self, x: isize, y: isize, flashed: &mut Vec<(isize, isize)>) -> bool {
        let is_valid = (0..self.width).contains(&x)
            && (0..self.height).contains(&y)
            && !flashed.contains(&(x, y));
        if is_valid {
            let index = (self.width * y + x) as usize;
            let value = self.data[index] + 1;
            self.data[index] = value;
            if value > 9 {
                self.data[index] = 0;
                flashed.push((x, y));
                for (dx, dy) in [
                    (x - 1, y),
                    (x + 1, y),
                    (x, y - 1),
                    (x, y + 1),
                    (x - 1, y - 1),
                    (x + 1, y + 1),
                    (x + 1, y - 1),
                    (x - 1, y + 1),
                ] {
                    self.iterate_octopus(dx, dy, flashed);
                }
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let data = input
        .lines()
        .flat_map(|line| {
            line.as_bytes()
                .iter()
                .map(|b| b - b'0')
                .collect::<Vec<u8>>()
        })
        .collect();
    Grid::new(
        input
            .lines()
            .next()
            .map_or_else(|| 0, |line| line.len() as isize),
        input.lines().count() as isize,
        data,
    )
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(11, use_sample)?;
    let mut data = parse_input(&input);
    let flashes = timed_run!("Part 1", data.part_1(100));
    println!("Number of flashes {}", flashes);
    let mut data = parse_input(&input);
    let step = timed_run!("Part 2", data.part_2(&[]));
    println!("First step is {}", step);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(11, true).unwrap();
        let mut data = parse_input(&input);
        let flashes = data.part_1(100);
        assert_eq!(flashes, 1656);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(11, true).unwrap();
        let mut data = parse_input(&input);
        let step = data.part_2(&[]);
        assert_eq!(step, 195);
    }
}
