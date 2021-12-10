use std::env::args;

use advent::{read_input, timed_run, AdventResult};

#[derive(Debug, Default)]
struct Grid {
    size: (isize, isize),
    data: Vec<i8>,
}

impl Grid {
    fn new(size: (isize, isize), data: Vec<i8>) -> Self {
        Self {
            size,
            data,
        }
    }

    fn simple_kernel(&mut self, x: isize, y: isize) -> Option<(i8, usize)> {
        let is_valid = (0..self.size.0).contains(&x) && (0..self.size.1).contains(&y);
        (is_valid && self.data[(y * self.size.0 + x) as usize] < 9).then(|| {
            let mut lowest = self.data[(y * self.size.0 + x) as usize];
            // Default basin size
            let mut size = 1;
            // Since we visited this tag it
            self.data[(self.size.0 * y + x) as usize] = 9;
            // Check the cross elements
            for (dx, dy) in [(x - 1, y), (x + 1, y), (x, y + 1), (x, y - 1)] {
                if let Some((v, s)) = self.simple_kernel(dx, dy) {
                    size += s;
                    lowest = lowest.min(v);
                }
            }
            (lowest, size)
        })
    }

    fn basins(&mut self) -> Vec<(i8, usize)> {
        let mut basins_sizes = vec![];
        for y in 0..self.size.1 {
            for x in 0..self.size.0 {
                if let Some(result) = self.simple_kernel(x, y) {
                    basins_sizes.push(result);
                }
            }
        }
        basins_sizes.sort_unstable_by_key(|k| k.1);
        basins_sizes
    }

    fn risk_level(&mut self) -> u32 {
        self.basins().iter().map(|(v, _)| (v + 1) as u32).sum()
    }

    fn biggest_basins_product(&mut self) -> usize {
        let basins = self.basins();
        basins.iter().skip(basins.len() - 3).map(|(_, size)| size).product()
    }
}

fn parse_input(input: &str) -> AdventResult<Grid> {
    let mut data = vec![];
    let width = input.lines().next().map_or_else(|| 0, |l| l.len()) as isize;
    let height = input.lines().count() as isize;
    for line in input.lines() {
        data.append(&mut line.bytes().map(|n| (n - b'0') as i8).collect());
    }
    Ok(Grid::new((width, height), data))
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(9, use_sample)?;
    let mut data = parse_input(&input)?;
    let result = timed_run!("Part 1", data.risk_level());
    println!("Risk level is: {}", result);
    let mut data = parse_input(&input)?;
    let result = timed_run!("Part 2", data.biggest_basins_product());
    println!("3 biggest basins product: {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(9, true).unwrap();
        let mut data = parse_input(&input).unwrap();
        assert_eq!(data.risk_level(), 15);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(9, true).unwrap();
        let mut data = parse_input(&input).unwrap();
        assert_eq!(data.biggest_basins_product(), 1134);
    }
}
