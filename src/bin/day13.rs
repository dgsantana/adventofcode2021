use std::env::args;

use advent::{read_input, timed_run, AdventResult};

#[derive(Debug)]
enum Fold {
    X(u32),
    Y(u32),
}

impl Fold {
    fn fold(&self, data: &mut Vec<(u32, u32)>) -> usize {
        match self {
            Fold::X(pos) => self.fold_vertical(*pos, data),
            Fold::Y(pos) => self.fold_horizontal(*pos, data),
        }
    }

    fn fold_vertical(&self, pos: u32, data: &mut Vec<(u32, u32)>) -> usize {
        data.iter_mut()
            .filter(|(x, _)| *x > pos)
            .for_each(|(x, _)| {
                *x = pos - (*x - pos);
            });
        de_dup(data);
        data.len()
    }

    fn fold_horizontal(&self, pos: u32, data: &mut Vec<(u32, u32)>) -> usize {
        data.iter_mut()
            .filter(|(_, y)| *y > pos)
            .for_each(|(_, y)| {
                *y = pos - (*y - pos);
            });
        de_dup(data);
        data.len()
    }
}

fn de_dup(data: &mut Vec<(u32, u32)>) {
    let mut seen = Vec::with_capacity(data.len() / 2);
    data.retain(|item| match seen.contains(item) {
        true => false,
        false => {
            seen.push(*item);
            true
        }
    });
}

fn parse_input(input: &str) -> (Vec<(u32, u32)>, Vec<Fold>) {
    let mut dots = vec![];
    let mut folds = vec![];
    for line in input.lines() {
        if line.starts_with("fold") {
            if let Some(fold) = line.split_ascii_whitespace().last() {
                let parts = fold.split('=').collect::<Vec<&str>>();
                match parts[0] {
                    "x" => folds.push(Fold::X(parts[1].parse::<u32>().unwrap_or_default())),
                    "y" => folds.push(Fold::Y(parts[1].parse::<u32>().unwrap_or_default())),
                    _ => (),
                }
            }
        } else {
            let point = line
                .split(',')
                .filter_map(|part| part.parse::<u32>().ok())
                .collect::<Vec<u32>>();
            if point.len() == 2 {
                dots.push((point[0], point[1]));
            }
        }
    }
    (dots, folds)
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(13, use_sample)?;
    let (mut data, folds) = parse_input(&input);
    let first = folds.first().unwrap();
    let count = timed_run!("Part 1", first.fold(&mut data));
    println!("Number of unique dots: {}", count);
    timed_run!("Part 2", {
        for fold in folds.iter().skip(1) {
            fold.fold(&mut data);
        }
    });
    display(&data, None);
    Ok(())
}

fn display(data: &[(u32, u32)], fold: Option<&Fold>) {
    let max_x = data.iter().map(|(x, _)| *x).max().unwrap();
    let mut max_y = data.iter().map(|(_, y)| *y).max().unwrap();
    if let Some(Fold::Y(pos)) = fold {
        if *pos > max_y {
            max_y = *pos - 1;
        }
    };
    for y in 0..=max_y {
        if let Some(Fold::Y(pos)) = fold {
            if *pos == y {
                println!("{}", "-".repeat(max_x as usize));
                continue;
            }
        }
        for x in 0..=max_x {
            if data.contains(&(x, y)) {
                print!("\u{2588}");
            } else if let Some(Fold::X(pos)) = fold {
                if *pos == x {
                    print!("|");
                    continue;
                }
            } else {
                print!("\x1B[38;5;8m\u{2588}\x1B[0m");
            }
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(13, true).unwrap();
        let (mut data, folds) = parse_input(&input);
        let first = folds.first().unwrap();
        assert_eq!(first.fold(&mut data), 17);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(13, true).unwrap();
    }
}
