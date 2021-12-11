#[derive(Debug)]
struct Grid<T> {
    grid: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = Vec<T>;

    fn index(&self, index: usize) -> &Self::Output {
        &self.grid[index]
    }
}

impl<T> std::ops::IndexMut<usize> for Grid<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.grid[index]
    }
}

impl<T> From<Vec<Vec<T>>> for Grid<T> {
    fn from(grid: Vec<Vec<T>>) -> Self {
        let (height, width) = (grid.len(), grid[0].len());
        Grid {
            grid,
            height,
            width,
        }
    }
}

impl<T: Clone + Default> Grid<T> {
    fn new_default(height: usize, width: usize) -> Grid<T> {
        Grid {
            grid: vec![vec![T::default(); width]; height],
            height,
            width,
        }
    }
}

#[derive(Debug)]
struct Simulation {
    grid: Grid<u8>,
    flash_counter: usize,
    first_sync: Option<usize>,
    counter: usize,
}

impl From<Grid<u8>> for Simulation {
    fn from(grid: Grid<u8>) -> Self {
        Simulation {
            grid,
            flash_counter: 0usize,
            first_sync: None,
            counter: 0usize,
        }
    }
}

impl Simulation {
    fn tick(&mut self) {
        let mut flashed: Grid<bool> = Grid::new_default(self.grid.height, self.grid.width);
        let mut expansion_queue = vec![];
        let mut tick_flashes = 0usize;
        for i in 0..self.grid.height {
            for j in 0..self.grid.width {
                self.increment(i, j, &mut flashed, &mut tick_flashes, &mut expansion_queue);
            }
        }
        while let Some((x, y)) = expansion_queue.pop() {
            for (i, j) in adjacent_indices(x, y, self.grid.height, self.grid.width) {
                if !flashed.grid[i][j] {
                    self.increment(i, j, &mut flashed, &mut tick_flashes, &mut expansion_queue);
                }
            }
        }
        self.flash_counter += tick_flashes;
        if tick_flashes == self.grid.width * self.grid.height {
            self.first_sync = Some(self.counter);
        }
        self.counter += 1;
    }

    fn increment(
        &mut self,
        x: usize,
        y: usize,
        flashed: &mut Grid<bool>,
        tick_flashes: &mut usize,
        queue: &mut Vec<(usize, usize)>,
    ) {
        self.grid[x][y] += 1;
        if self.grid[x][y] >= 10 {
            flashed[x][y] = true;
            self.grid[x][y] = 0;
            *tick_flashes += 1;
            queue.push((x, y));
        }
    }
}

fn adjacent_indices(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let min_x = if let Some(x) = x.checked_sub(1) { x } else { 0 };
    let max_x = if x + 1 < height { x + 1 } else { x };
    let min_y = if let Some(y) = y.checked_sub(1) { y } else { 0 };
    let max_y = if y + 1 < width { y + 1 } else { y };
    let mut acc = vec![];
    for i in min_x..=max_x {
        for j in min_y..=max_y {
            if i != x || j != y {
                acc.push((i, j));
            }
        }
    }
    acc
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid: Grid<u8> = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>()
        .into();
    let mut sim: Simulation = grid.into();
    (0..100usize).for_each(|_| sim.tick());
    println!("A: {}", sim.flash_counter);
    while sim.first_sync == None {
        sim.tick();
    }
    println!("B: {}", sim.first_sync.unwrap() + 1);
    Ok(())
}
