use std::{
    env::args,
    io::{BufReader, Read},
};

use advent::AdventResult;

const SAMPLE: &str = "3,4,3,1,2";

fn parser(input: &[u8]) -> AdventResult<Vec<u8>> {
    let mut reader = BufReader::new(input);
    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;
    Ok(buffer
        .trim()
        .split(',')
        .filter_map(|n| n.trim().parse::<u8>().ok())
        .collect())
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
    let input = if use_sample {
        SAMPLE.as_bytes()
    } else {
        include_bytes!("../../day6.txt")
    };
    let lantern_fish = parser(input)?;
    let (count, days) = if slow {
        (growth_lazy(&lantern_fish, 18), 18)
    } else {
        (growth(&lantern_fish, 256), 256)
    };
    println!("Total of lantern fishes after {} days is {}", days, count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{growth, growth_lazy, parser, SAMPLE};

    #[test]
    fn lazy_growth() {
        let latern_fish = parser(SAMPLE.as_bytes()).expect("Invalid data.");
        let count = growth_lazy(&latern_fish, 80);
        assert_eq!(count, 5934);
    }

    #[test]
    fn fast_growth() {
        let latern_fish = parser(SAMPLE.as_bytes()).expect("Invalid data.");
        let count = growth(&latern_fish, 256);
        assert_eq!(count, 26984457539);
    }
}
