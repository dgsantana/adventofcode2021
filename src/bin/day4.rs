use std::{env::args, fmt::Display};

use advent::{read_input, AdventError, AdventResult, timed_run};

#[derive(Debug, Default)]
struct BoardNumber {
    value: u32,
    marked: bool,
    position: (usize, usize),
}

impl BoardNumber {
    fn new(value: u32, position: (usize, usize)) -> Self {
        Self {
            value,
            marked: false,
            position,
        }
    }
}

type Winner = (u32, Vec<u32>, usize, bool);

#[derive(Debug, Default)]
struct Board {
    number: usize,
    rank: u32,
    size: (usize, usize),
    numbers: Vec<BoardNumber>,
    winner: Option<Winner>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.numbers.len() {
            if (i % self.size.0 == 0 || i == self.size.1) && i != 0 {
                writeln!(f)?;
            }
            if self.numbers[i].marked {
                write!(f, "*")?;
            } else {
                write!(f, " ")?;
            }
            write!(f, "{:02} ", self.numbers[i].value)?;
        }
        writeln!(f)
    }
}

impl Board {
    fn new(number: usize, size: (usize, usize), numbers: Vec<BoardNumber>) -> Self {
        Self {
            number,
            rank: 0,
            size,
            numbers,
            winner: None,
        }
    }

    fn mark(&mut self, drawn_number: u32) -> bool {
        for number in self.numbers.iter_mut() {
            if drawn_number == number.value {
                number.marked = true;
            }
        }
        let mut winner = false;
        // Check rows
        if let Some((index, row)) = self
            .numbers
            .chunks(self.size.0)
            .enumerate()
            .find(|(_, row)| row.iter().all(|n| n.marked))
        {
            let unmarked_sum = self
                .numbers
                .iter()
                .filter(|n| !n.marked)
                .map(|n| n.value)
                .sum::<u32>();
            self.winner = Some((
                unmarked_sum * drawn_number,
                row.iter().map(|n| n.value).collect(),
                index,
                true,
            ));
            winner = true;
        } else {
            // Check columns
            for i in 0..self.size.0 {
                let (column, _): (Vec<&BoardNumber>, Vec<&BoardNumber>) =
                    self.numbers.iter().partition(|n| n.position.0 == i);
                if column.iter().all(|n| n.marked) {
                    let unmarked_sum = self
                        .numbers
                        .iter()
                        .filter(|n| !n.marked)
                        .map(|n| n.value)
                        .sum::<u32>();
                    self.winner = Some((
                        unmarked_sum * drawn_number,
                        column.iter().map(|n| n.value).collect(),
                        i,
                        false,
                    ));
                    winner = true;
                    break;
                }
            }
        }
        winner
    }
}

#[derive(Debug, Default)]
struct Bingo {
    boards: Vec<Board>,
    numbers: Vec<u32>,
}

impl Bingo {
    fn new(boards: Vec<Board>, numbers: Vec<u32>) -> Self {
        Self { boards, numbers }
    }

    fn parse_input(input: &str) -> AdventResult<Bingo> {
        // Read drawn numbers
        let drawn_numbers: Vec<u32> = input
            .lines()
            .next()
            .ok_or(AdventError::InvalidData)?
            .split(',')
            .into_iter()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        let mut boards = Vec::<Board>::new();
        let mut rows = 0;
        let mut columns = 0;
        let mut board_numbers = Vec::<BoardNumber>::new();
        let mut board = 0;

        for line in input.lines().skip(2) {
            let clean_buffer = line.trim();
            if clean_buffer.is_empty() {
                boards.push(Board::new(board, (columns, rows), board_numbers));
                board += 1;
                board_numbers = Vec::new();
                columns = 0;
                rows = 0;
            } else {
                let mut numbers: Vec<BoardNumber> = clean_buffer
                    .split(' ')
                    .into_iter()
                    .filter_map(|n| n.parse::<u32>().ok())
                    .enumerate()
                    .map(|(index, n)| BoardNumber::new(n, (index, rows)))
                    .collect();
                if columns == 0 {
                    columns = numbers.len();
                }
                board_numbers.append(&mut numbers);
                rows += 1;
            }
        }
        // Add the last board
        boards.push(Board::new(board, (columns, rows), board_numbers));
        Ok(Self::new(boards, drawn_numbers))
    }

    fn rank_winners(&mut self) {
        let mut rank = 1;
        for &number in self.numbers.iter() {
            for board in self.boards.iter_mut().filter(|b| b.winner.is_none()) {
                if board.mark(number) {
                    board.rank = rank;
                    rank += 1;
                }
            }
        }
        self.boards.sort_by_key(|b| b.rank);
    }

    fn first_winner(&self) -> Option<&Board> {
        self.boards.iter().find(|b| b.rank == 1)
    }

    fn last_winner(&self) -> Option<&Board> {
        self.boards.last()
    }
}

fn main() -> AdventResult<()> {
    let use_sample = args().any(|arg| arg == "--sample");
    let input = read_input(4, use_sample)?;
    let mut bingo = Bingo::parse_input(&input)?;
    timed_run!("Part 1 and 2",bingo.rank_winners());

    let winner = bingo.first_winner();
    let last_winner = bingo.last_winner();

    if let Some(board) = winner {
        let winner = board.winner.as_ref().unwrap();
        print!("First winner on board {} ", board.number + 1);
        if winner.3 {
            print!("row ");
        } else {
            print!("column ");
        }
        println!(
            "{} with this values {:?} with score {}",
            winner.2 + 1,
            winner.1,
            winner.0
        );
        println!("{}", board);
    }

    if let Some(board) = last_winner {
        let winner = board.winner.as_ref().unwrap();
        print!(
            "Last winner on board {} rank {} ",
            board.number + 1,
            board.rank
        );
        if winner.3 {
            print!("row ");
        } else {
            print!("column ");
        }
        println!(
            "{} with this values {:?} with score {}",
            winner.2 + 1,
            winner.1,
            winner.0
        );
        println!("{}", board);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn validate_row_winner() {
        let input = read_input(4, true).expect("Error reading input");
        let mut bingo = Bingo::parse_input(&input).expect("Invalid data");
        bingo.rank_winners();
        let result = bingo.first_winner().unwrap();
        let winner = result.winner.as_ref().unwrap();
        assert_eq!(winner.0, 4512);
    }

    #[test]
    fn validate_last_winner() {
        let input = read_input(4, true).expect("Error reading input");
        let mut bingo = Bingo::parse_input(&input).expect("Invalid data");
        bingo.rank_winners();
        let result = bingo.last_winner().unwrap();
        let winner = result.winner.as_ref().unwrap();
        assert_eq!(winner.0, 1924);
    }
}
