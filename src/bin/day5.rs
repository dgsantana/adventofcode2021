use std::{
    collections::HashMap,
    env::args,
    fmt::Display,
    io::{BufRead, BufReader},
};

use advent::AdventResult;

const SAMPLE: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2";

/// Simple point, since it's just u32, we are going to allow Copy and Clone.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: u32,
    y: u32,
}

/// Just to get beauty prints.
impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

/// Line segment
#[derive(Debug, Default)]
struct LineSegment {
    p1: Point,
    p2: Point,
}

/// Just to get beauty prints.
impl Display for LineSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.p1, self.p2)
    }
}

impl LineSegment {
    fn new(p1: Point, p2: Point) -> Self {
        Self { p1, p2 }
    }

    /// This uses the full algorithm for Bresenham lines
    ///
    /// [Wikipedia](https://en.wikipedia.org/wiki/Bresenham's_line_algorithm)
    fn bresenham_line_points(&self) -> Vec<Point> {
        let dx = (self.p2.x as i64 - self.p1.x as i64).abs();
        let dy = -(self.p2.y as i64 - self.p1.y as i64).abs();
        let mut err = dx + dy;
        let mut x = self.p1.x;
        let mut y = self.p1.y;

        let mut points = vec![];
        loop {
            points.push(Point { x, y });
            if x == self.p2.x && y == self.p2.y {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                if self.p1.x < self.p2.x {
                    x += 1;
                } else {
                    x -= 1;
                }
            }
            if e2 <= dx {
                err += dx;
                if self.p1.y < self.p2.y {
                    y += 1;
                } else {
                    y -= 1;
                }
            }
        }
        points
    }
}

/// Our grid where we store the lines and overlap information.
#[derive(Debug, Default)]
struct Grid {
    lines: Vec<LineSegment>,
    size: (u32, u32),
    accumulator: HashMap<Point, u32>,
}

impl Grid {
    fn insert_line_orthogonal_trace(&mut self, line: LineSegment) {
        self.size.0 = self.size.0.max(line.p1.x).max(line.p2.x);
        self.size.1 = self.size.1.max(line.p1.y).max(line.p2.y);
        self.trace_orthogonal_lines(&line);
        self.lines.push(line);
    }

    fn insert_line(&mut self, line: LineSegment) {
        self.size.0 = self.size.0.max(line.p1.x).max(line.p2.x);
        self.size.1 = self.size.1.max(line.p1.y).max(line.p2.y);
        self.trace_lines(&line);
        self.lines.push(line);
    }

    fn trace_orthogonal_lines(&mut self, line: &LineSegment) {
        if line.p1.x == line.p2.x {
            let x = line.p1.x;
            let ymin = line.p1.y.min(line.p2.y);
            let ymax = line.p1.y.max(line.p2.y);
            for y in ymin..=ymax {
                self.accumulator
                    .entry(Point { x, y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
        if line.p1.y == line.p2.y {
            let y = line.p1.y;
            let x_min = line.p1.x.min(line.p2.x);
            let x_max = line.p1.x.max(line.p2.x);
            for x in x_min..=x_max {
                self.accumulator
                    .entry(Point { x, y })
                    .and_modify(|v| *v += 1)
                    .or_insert(1);
            }
        }
    }

    fn trace_lines(&mut self, line: &LineSegment) {
        let points = line.bresenham_line_points();
        for point in points {
            self.accumulator
                .entry(point)
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }
    }

    fn overlaps(&self) -> u32 {
        self.accumulator.iter().filter(|(_, &v)| v >= 2).count() as u32
    }

    fn draw_lines_data(&self) {
        for line in &self.lines {
            print!("{} [", &line);
            for point in &line.bresenham_line_points() {
                print!("({}),", point);
            }
            println!("]");
        }
    }

    fn draw_grid(&self) {
        for y in 0..=self.size.1 {
            for x in 0..=self.size.0 {
                let p = Point { x, y };
                let v = if let Some((_, a)) = self.accumulator.get_key_value(&p) {
                    format!("{}", a)
                } else {
                    ".".to_owned()
                };
                print!("{}", v);
            }
            println!();
        }
    }
}

/// Parses the lines and adds them to the grid.
/// 
/// If `orthogonal` is true, only horizontal or vertical lines are check for overlaps.
/// This is for the first part.
fn line_parser(input: &[u8], orthogonal: bool) -> Grid {
    let mut reader = BufReader::new(input);
    let mut buffer = String::with_capacity(11);

    let mut grid = Grid::default();

    while let Ok(size) = reader.read_line(&mut buffer) {
        if size > 0 {
            let clean_buffer = buffer.trim();
            let points_str = clean_buffer.split("->");
            let points = points_str
                .map(|p| {
                    p.trim()
                        .split(',')
                        .filter_map(|n| n.parse::<u32>().ok())
                        .collect::<Vec<u32>>()
                })
                .filter_map(|p| {
                    if p.len() != 2 {
                        None
                    } else {
                        Some(Point { x: p[0], y: p[1] })
                    }
                })
                .collect::<Vec<Point>>();
            if points.len() != 2 {
                println!("Invalid line '{}'.", &clean_buffer);
                continue;
            }
            let line = LineSegment::new(points[0], points[1]);
            if orthogonal {
                grid.insert_line_orthogonal_trace(line);
            } else {
                grid.insert_line(line);
            }
            buffer.clear();
        } else {
            break;
        }
    }
    grid
}

/// To get a nice diagram for the sample data use the --sample argument
fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = if use_sample {
        SAMPLE.as_bytes()
    } else {
        include_bytes!("../../day5.txt")
    };

    let grid = line_parser(input, false);
    println!("Grid size: {}x{}", &grid.size.0, &grid.size.1);
    if use_sample {
        grid.draw_lines_data();
        grid.draw_grid();
    }
    println!("Overlaps: {}", grid.overlaps());
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{line_parser, SAMPLE};

    #[test]
    fn validate_overlaps_orthogonal() {
        let input = SAMPLE.as_bytes();
        let grid = line_parser(input, true);
        assert_eq!(grid.overlaps(), 5);
    }

    #[test]
    fn validate_overlaps() {
        let input = SAMPLE.as_bytes();
        let grid = line_parser(input, false);
        assert_eq!(grid.overlaps(), 12);
    }
}
