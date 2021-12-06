use std::{error::Error, usize};

const SIMULATION_TICKS_A: usize = 80;
const SIMULATION_TICKS_B: usize = 256;
const UPPER_CICLE_LIMIT: usize = 9;
const BIRTH_IDX: usize = 8;
const POST_REPR_IDX: usize = 6;

struct LanternFishes {
    cells: [usize; UPPER_CICLE_LIMIT],
}

impl LanternFishes {
    fn new(initial: &[usize]) -> LanternFishes {
        let mut cells = [0usize; UPPER_CICLE_LIMIT];
        initial.iter().for_each(|x| cells[*x] += 1);
        LanternFishes { cells }
    }

    fn tick(&mut self) {
        let mut temp = [0usize; UPPER_CICLE_LIMIT];
        (1..UPPER_CICLE_LIMIT).for_each(|i| temp[i - 1] = self.cells[i]);
        temp[BIRTH_IDX] = self.cells[0];
        temp[POST_REPR_IDX] += self.cells[0];
        self.cells = temp;
    }

    fn sum(self) -> usize {
        self.cells.into_iter().sum::<usize>()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = String::from_utf8(std::fs::read("input.txt")?)?;

    let entities = input
        .as_str()
        .split(',')
        .map(|s| s.parse::<usize>())
        .filter_map(|r| r.ok())
        .collect::<Vec<usize>>();

    let mut fishes = LanternFishes::new(entities.as_slice());
    (0..SIMULATION_TICKS_A).for_each(|_| fishes.tick());
    println!("{} days: {}", SIMULATION_TICKS_A, fishes.sum());

    let mut fishes = LanternFishes::new(entities.as_slice());
    (0..SIMULATION_TICKS_B).for_each(|_| fishes.tick());
    println!("{} days: {}", SIMULATION_TICKS_B, fishes.sum());

    Ok(())
}
