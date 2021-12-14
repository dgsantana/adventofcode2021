use std::{collections::HashMap, env::args};

use advent::{read_input, timed_run, AdventResult};

#[derive(Debug, Default)]
struct Synthesis<'a> {
    template: &'a str,
    rules: HashMap<(u8, u8), u8>,
}

impl<'a> Synthesis<'a> {
    fn grow_brute_force(&self, cycles: usize) -> (usize, Vec<u8>) {
        let mut counter = HashMap::<u8, usize>::new();
        let mut chain = self.template.as_bytes().to_vec();

        for &b in chain.iter() {
            *counter.entry(b).or_insert(0) += 1;
        }

        for _ in 0..cycles {
            let pairs: Vec<_> = chain
                .iter()
                .zip(chain.iter().skip(1))
                .map(|(&a, &b)| [a, b])
                .collect();
            let mut position = 1;
            for pair in pairs.iter() {
                for (rule_pair, &element) in self.rules.iter() {
                    if pair[0] == rule_pair.0 && pair[1] == rule_pair.1 {
                        chain.insert(position, element);
                        *counter.entry(element).or_insert(0) += 1;
                        position += 2;
                    }
                }
            }
            #[cfg(feature = "info_prints")]
            display(&chain);
        }
        let min = counter.values().min().unwrap();
        let max = counter.values().max().unwrap();

        ((max - min), chain)
    }

    /// Smarter grow  
    /// The main drawback is that we can't get the produced chain
    /// we only get the number of pairs, but it is very efficient.
    fn grow(&self, cycles: usize) -> usize {
        let mut counter = HashMap::<u8, usize>::new();
        let mut chain = HashMap::<(u8, u8), usize>::new();

        // Count our input template
        for b in self.template.bytes() {
            *counter.entry(b).or_insert(0) += 1;
        }

        // Prepare the first pairs
        for pair in self.template.bytes().zip(self.template.bytes().skip(1)) {
            *chain.entry(pair).or_insert(0) += 1;
        }

        // Drain keeps the current capacity, so we have a constant alloc.
        for _ in 0..cycles {
            let pairs: Vec<_> = chain.drain().filter(|(_, count)| *count > 0).collect();
            for (pair, pair_count) in pairs.iter() {
                if *pair_count > 0 {
                    if let Some(&element) = self.rules.get(pair) {
                        *counter.entry(element).or_insert(0) += pair_count;
                        *chain.entry((pair.0, element)).or_insert(0) += pair_count;
                        *chain.entry((element, pair.1)).or_insert(0) += pair_count;
                    }
                }
            }
        }
        let min = counter.values().min().unwrap();
        let max = counter.values().max().unwrap();

        max - min
    }
}

#[cfg(feature = "info_prints")]
fn display(chain: &[u8]) {
    println!("{}", String::from_utf8_lossy(chain))
}

#[cfg(feature = "info_prints")]
fn display_pairs(chain: &HashMap<(u8, u8), usize>) {
    for (pair, size) in chain {
        println!("{} {}", String::from_utf8_lossy(&[pair.0, pair.1]), size);
    }
}

fn parse_input(input: &str) -> Synthesis<'_> {
    let mut synthesis = Synthesis::default();
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        if line.contains("->") {
            if let Some((a, b)) = line.split_once("->") {
                let pair = a.trim().as_bytes();
                synthesis
                    .rules
                    .entry((pair[0], pair[1]))
                    .or_insert_with(|| b.trim().as_bytes()[0]);
            }
        } else {
            synthesis.template = line;
        }
    }
    synthesis
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(14, use_sample)?;
    let data = parse_input(&input);
    let result = timed_run!("Part 1", data.grow_brute_force(10));
    println!(
        "Chain size is {} and a max-min = {}",
        result.1.len(),
        result.0
    );
    let result = timed_run!("Part 2", data.grow(40));
    println!("After 40 cycles (max-min) = {}", result);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(14, true).unwrap();
        let data = parse_input(&input);
        let result = data.grow_brute_force(10);
        assert_eq!(result.0, 1588);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(14, true).unwrap();
        let data = parse_input(&input);
        let result = data.grow(40);
        assert_eq!(result, 2188189693529);
    }
}
