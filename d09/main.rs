use std::error::Error;

#[derive(Debug)]
struct Grid<T> {
    cells: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T> Grid<T> {
    pub fn new(cells: Vec<Vec<T>>) -> Grid<T> {
        let (height, width) = (cells.len(), cells[0].len());
        Grid {
            cells,
            height,
            width,
        }
    }
}

impl<T> Grid<T>
where
    T: Default + Clone,
{
    pub fn new_default(height: usize, width: usize) -> Grid<T> {
        let cells = vec![vec![T::default(); width]; height];
        Grid {
            cells,
            height,
            width,
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let grid = Grid::new(
        String::from_utf8(std::fs::read("input.txt")?)?
            .lines()
            .map(|l| l.chars().map(|c| (c as u8 - b'0')).collect::<Vec<u8>>())
            .collect::<Vec<Vec<u8>>>(),
    );
    println!(
        "Part A: {}",
        find_lowest(&grid)
            .into_iter()
            .map(|(x, y)| (grid.cells[x][y] as u16) + 1)
            .sum::<u16>()
    );

    // --- Part Two --- //

    let mut res = find_lowest(&grid)
        .into_iter()
        .map(|(x, y)| find_max_basin_size_with_lowest(x, y, &grid))
        .collect::<Vec<u16>>();
    res.sort_by(|a, b| b.cmp(a));
    let three_major_mul = &res[..3]
        .iter()
        .map(|x| *x as u32)
        .reduce(|acc, x| acc * x)
        .unwrap();
    println!("Part B: {}", three_major_mul);

    Ok(())
}

fn find_max_basin_size_with_lowest(x: usize, y: usize, grid: &Grid<u8>) -> u16 {
    let mut mask: Grid<bool> = Grid::new_default(grid.height, grid.width);
    let mut max = 0u16;
    find_max_basin_size_with_lowest_recv(x, y, grid, &mut mask, &mut max);
    max
}

fn find_max_basin_size_with_lowest_recv(
    x: usize,
    y: usize,
    grid: &Grid<u8>,
    mask: &mut Grid<bool>,
    max_size: &mut u16,
) {
    mask.cells[x][y] = true;
    *max_size += 1;
    for (i, j) in adj_idx_4dir(x, y, grid.height, grid.width) {
        if !mask.cells[i][j] && grid.cells[i][j] != 9 {
            find_max_basin_size_with_lowest_recv(i, j, grid, mask, max_size);
        }
    }
}

fn find_lowest(grid: &Grid<u8>) -> Vec<(usize, usize)> {
    let mut acc = vec![];
    for i in 0..grid.height {
        for j in 0..grid.width {
            if !adj_idx_4dir(i, j, grid.height, grid.width)
                .into_iter()
                .any(|(x, y)| grid.cells[x][y] <= grid.cells[i][j])
            {
                acc.push((i, j));
            }
        }
    }
    acc
}

fn adj_idx_4dir(x: usize, y: usize, height: usize, width: usize) -> Vec<(usize, usize)> {
    let mut acc = vec![];
    if let Some(x) = x.checked_sub(1) {
        acc.push((x, y));
    }
    if let Some(y) = y.checked_sub(1) {
        acc.push((x, y));
    }
    if x + 1 < height {
        acc.push((x + 1, y));
    }
    if y + 1 < width {
        acc.push((x, y + 1));
    }
    acc
}
