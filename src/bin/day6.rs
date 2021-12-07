use std::env::args;

use advent::{read_input, AdventResult, timed_run};

fn parse_input(input: &str) -> Vec<u8> {
    input
        .trim()
        .split(',')
        .filter_map(|n| n.trim().parse::<u8>().ok())
        .collect()
}

#[allow(dead_code)]
fn growth_lazy(fishes: &[u8], days: usize) -> u64 {
    let mut fish_pool = Vec::from(fishes);

    #[cfg(info_prints)]
    println!("  Initial state: {:?}", &fishes);
    #[allow(unused_variables)]
    for i in 0..days {
        for j in 0..fish_pool.len() {
            if fish_pool[j] == 0 {
                fish_pool[j] = 6;
                fish_pool.push(8);
            } else {
                fish_pool[j] -= 1;
            }
        }
        #[cfg(info_prints)]
        println!("After {:2} day(s): {:?}", i + 1, &fish_pool);
    }
    fish_pool.len() as u64
}

/// Rotation based stage totals, avoids the lazy method that over-allocates the vector.
fn growth(fishes: &[u8], days: usize) -> u64 {
    let mut fish_states = [0_u64; 9];
    // Build growth stage totals
    fishes.iter().for_each(|a| fish_states[*a as usize] += 1);
    #[cfg(info_prints)]
    println!("  Initial state: {:?}", &fishes);
    // Diagram for sample fishes ages 3, 4, 3, 1, 2
    // Age stages   0  1  2  3  4  5  6  7  8
    // Day 1 Stages 1, 1, 2, 1, 0, 0, 0, 0, 0 Fishes ages: 2, 3, 2, 0, 1
    // Day 2 Stages 1, 2, 1, 0, 0, 0, 1, 0, 1 Fishes ages: 1, 2, 1, 6, 0, 8
    // We rotate left, moving all fishes from stage 0 to 8 and as so we need to also add them to the stage 6 fishes
    // that were 0. The rotation simulates the days passing.
    #[allow(unused_variables)]
    for i in 0..days {
        fish_states.rotate_left(1);
        fish_states[6] += fish_states[8];
        #[cfg(info_prints)]
        println!("After {:2} day(s): {:?}", i + 1, &fish_states);
    }
    fish_states.iter().sum()
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let slow = args().any(|arg| arg == "--slow");
    let input = read_input(6, use_sample)?;
    let lantern_fish = parse_input(&input);
    let (count, days) = if slow {
        timed_run!("Part 1 lazy", (growth_lazy(&lantern_fish, 18), 18))
    } else {
        timed_run!("Part 2", (growth(&lantern_fish, 256), 256))
    };
    println!("Total of lantern fishes after {} days is {}", days, count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lazy_growth() {
        let input = read_input(6, true).expect("Error reading input");
        let lantern_fish = parse_input(&input);
        let count = growth_lazy(&lantern_fish, 80);
        assert_eq!(count, 5934);
    }

    #[test]
    fn fast_growth() {
        let input = read_input(6, true).expect("Error reading input");
        let lantern_fish = parse_input(&input);
        let count = growth(&lantern_fish, 256);
        assert_eq!(count, 26984457539);
    }
}
