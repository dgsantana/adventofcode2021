use std::{
    fmt::Display,
    io::{BufRead, BufReader},
};

use advent::AdventResult;

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

    fn parse(input: &[u8]) -> AdventResult<Bingo> {
        let mut reader = BufReader::new(input);
        let mut buffer = String::new();

        // Read drawn numbers
        reader.read_line(&mut buffer)?;
        let drawn_numbers: Vec<u32> = buffer
            .split(',')
            .into_iter()
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        // Skip one line
        reader.read_line(&mut buffer)?;
        buffer.clear();

        let mut boards = Vec::<Board>::new();
        let mut rows = 0;
        let mut columns = 0;
        let mut board_numbers = Vec::<BoardNumber>::new();
        let mut board = 0;

        while let Ok(size) = reader.read_line(&mut buffer) {
            if size > 0 {
                let clean_buffer = buffer.trim();
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
                buffer.clear();
            } else {
                boards.push(Board::new(board, (columns, rows), board_numbers));
                break;
            }
        }
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
    let input = include_bytes!("../../day4.txt");
    let mut bingo = Bingo::parse(input)?;
    bingo.rank_winners();

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
    use crate::Bingo;

    const SAMPLE_ROW: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

    const SAMPLE_COLUMN: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

 22 13 17 11  0
  8  2 23  4 24
 21  9 14 16  7
  6 10  3 18  5
  1 12 20 15 19
 
  3 15  0  2 22
  9 18 13 17  5
 19  8  7 25 23
 20 11 10 24  4
 14 21 16 12  6
 
 14 10 18 22  2
 21 16 15  9 19
 17 8 23 26 20
 24 11 13  6  5
 4 0 12 3  7";

    #[test]
    fn validate_row_winner() {
        let mut bingo = Bingo::parse(SAMPLE_ROW.as_bytes()).expect("Invalid data");
        bingo.rank_winners();
        let result = bingo.first_winner().unwrap();
        let winner = result.winner.as_ref().unwrap();
        assert_eq!(winner.0, 4512);
    }

    #[test]
    fn validate_column_winner() {
        let mut bingo = Bingo::parse(SAMPLE_COLUMN.as_bytes()).expect("Invalid data");
        bingo.rank_winners();
        let result = bingo.first_winner().unwrap();
        let winner = result.winner.as_ref().unwrap();
        assert_eq!(winner.0, 4512);
    }
}
