use std::collections::HashSet;
use std::error::Error;

const BOARD_DIM: usize = 5;

#[derive(Debug)]
struct Board {
    grid: [[u8; BOARD_DIM]; BOARD_DIM],
    unmarked: HashSet<(usize, usize)>,
    count: ([u8; BOARD_DIM], [u8; BOARD_DIM]),
}

impl Board {
    fn parse(input: &mut &str) -> Board {
        let mut grid = [[0u8; BOARD_DIM]; BOARD_DIM];
        let mut unmarked = HashSet::new();
        (0..BOARD_DIM).for_each(|i| {
            (0..BOARD_DIM).for_each(|j| {
                skip_whitespaces(input);
                let _ = unmarked.insert((i, j));
                let end = input.find(char::is_whitespace).unwrap_or(input.len());
                grid[i][j] = input[..end].parse::<u8>().unwrap();
                *input = &input[end..];
            });
        });
        Board {
            grid,
            unmarked,
            count: ([0u8; BOARD_DIM], [0u8; BOARD_DIM]),
        }
    }

    fn advance(&mut self, n: u8) -> bool {
        let mut idx = None;
        (0..BOARD_DIM).for_each(|i| {
            (0..BOARD_DIM).for_each(|j| {
                if self.grid[i][j] == n {
                    idx = Some((i, j));
                }
            })
        });
        if let Some((i, j)) = idx {
            if self.unmarked.contains(&(i, j)) {
                self.count.0[i] += 1;
                self.count.1[j] += 1;
                self.unmarked.remove(&(i, j));
                if self.count.0[i] >= BOARD_DIM as u8 || self.count.1[j] >= BOARD_DIM as u8 {
                    return true;
                }
            }
        }
        false
    }

    fn sum_unmarked(&mut self) -> usize {
        self.unmarked
            .iter()
            .map(|(i, j)| self.grid[*i][*j] as usize)
            .sum::<usize>()
    }
}

fn skip_whitespaces(input: &mut &str) {
    let end = input
        .find(|c: char| !c.is_whitespace())
        .unwrap_or_else(|| input.len());
    *input = &input[end..];
}

pub fn main() -> Result<(), Box<dyn Error>> {
    let input = String::from_utf8(std::fs::read("input.txt")?)?;
    let mut input_slice = input.as_str();

    let sequence = {
        let end = input_slice.find('\n').unwrap();
        let seq = input_slice[..end]
            .split(',')
            .map(|s| s.parse::<u8>().unwrap())
            .collect::<Vec<u8>>();
        input_slice = &input_slice[end..];
        seq
    };

    let mut boards = std::iter::from_fn(|| {
        skip_whitespaces(&mut input_slice);
        if input_slice.len() == 0 {
            None
        } else {
            Some(Board::parse(&mut input_slice))
        }
    })
    .collect::<Vec<Board>>();

    let mut first = None;
    let mut seq_remainder = 0usize;
    'out: for n in sequence.iter() {
        for b in 0..boards.len() {
            if boards[b].advance(*n) {
                first = Some((b, *n));
                break 'out;
            }
        }
        seq_remainder += 1;
    }
    let (board, n) = first.unwrap();

    let part_a = boards[board].sum_unmarked() * n as usize;

    let mut remaining = (0..boards.len()).collect::<Vec<usize>>();
    remaining.swap_remove(board);
    let mut last = None;

    for n in &sequence[seq_remainder..] {
        let mut completed = vec![];
        for b in 0..remaining.len() {
            if boards[remaining[b]].advance(*n) {
                completed.push(b);
                last = Some((remaining[b], *n));
            }
        }
        while let Some(idx) = completed.pop() {
            let _ = remaining.swap_remove(idx);
        }
    }

    let (board, n) = last.unwrap();
    let part_b = boards[board].sum_unmarked() * n as usize;

    println!("Part A: {}\nPart B: {}", part_a, part_b);
    Ok(())
}
