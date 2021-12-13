use std::{
    collections::{HashMap, HashSet},
    env::args,
};

use advent::{read_input, timed_run, AdventResult};

#[derive(Debug, Default)]
struct Node<'a> {
    children: Vec<&'a str>,
}

#[derive(Debug, Default)]
struct Graph<'a> {
    edges: HashMap<&'a str, Node<'a>>,
}

impl<'a> Graph<'a> {
    fn part_1(&'a self, paths: &mut HashSet<String>) -> u32 {
        self.find_path(&mut vec!["start"], false, false, paths)
    }

    fn part_2(&'a self, paths: &mut HashSet<String>) -> u32 {
        self.find_path(&mut vec!["start"], true, false, paths)
    }

    fn find_path(
        &'a self,
        path: &mut Vec<&'a str>,
        two_visits_allowed: bool,
        already_visited: bool,
        paths: &mut HashSet<String>,
    ) -> u32 {
        let current_node = path.iter().last().unwrap();
        let node = self.edges.get(current_node).unwrap();
        let mut count = 0;

        for &next in node.children.iter() {
            // Ignore any start node, we already pass it at the first call.
            if next == "start" {
                continue;
            }

            let mut visited = already_visited;
            if next.to_lowercase() == next && path.contains(&next) {
                if !two_visits_allowed || already_visited {
                    continue;
                } else {
                    visited = true;
                }
            }

            path.push(next);
            if next == "end" {
                #[cfg(feature="info_prints")]
                paths.push(path.join("-"));
                count += 1;
            } else {
                count += self.find_path(path, two_visits_allowed, visited, paths);
            }
            path.pop();
        }
        count
    }
}

fn parse_input(input: &str) -> Graph {
    let mut graph = Graph::default();
    for line in input.lines() {
        if let Some((a, b)) = line.split_once('-') {
            graph
                .edges
                .entry(a)
                .or_insert_with(Node::default)
                .children
                .push(b);
            graph
                .edges
                .entry(b)
                .or_insert_with(Node::default)
                .children
                .push(a);
        }
    }
    graph
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(12, use_sample)?;
    let data = parse_input(&input);
    let mut paths = HashSet::new();
    let count = timed_run!("Part 1", data.part_1(&mut paths));
    #[cfg(feature="info_prints")]
    for path in paths {
        println!("{}", path);
    }
    println!("Number of paths {}", count);

    let mut paths = HashSet::new();
    let count = timed_run!("Part 2", data.part_2(&mut paths));
    #[cfg(feature="info_prints")]
    for path in paths {
        println!("{}", path);
    }
    println!("Number of paths {}", count);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validate_part1() {
        let input = read_input(12, true).unwrap();
        let data = parse_input(&input);
        let mut paths = HashSet::new();
        assert_eq!(data.part_1(&mut paths), 10);
    }

    #[test]
    fn validate_part2() {
        let input = read_input(12, true).unwrap();
        let data = parse_input(&input);
        let mut paths = HashSet::new();
        assert_eq!(data.part_2(&mut paths), 36);
    }
}
