use std::ops::Add;

trait Grid2D<T> {
    fn size(&self) -> usize;
    fn get(&self, x: usize, y: usize) -> T;
}

impl<T> Grid2D<T> for Vec<Vec<T>>
where
    T: Copy,
{
    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, x: usize, y: usize) -> T {
        self[x][y]
    }
}

#[derive(Debug)]
struct ExtendedGrid<T> {
    grid: Vec<Vec<T>>,
    inner_size: usize,
    size: usize,
}

impl Grid2D<u8> for ExtendedGrid<u8> {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let (i, j) = (x % self.inner_size, y % self.inner_size);
        let to_add = (x / self.inner_size) + (y / self.inner_size);
        let r = self.grid[i][j] + to_add as u8;
        if r >= 10 {
            (r % 10) + 1
        } else {
            r
        }
    }
}

impl<T> ExtendedGrid<T> {
    fn new(grid: Vec<Vec<T>>) -> ExtendedGrid<T> {
        let inner_size = grid.len();
        ExtendedGrid {
            grid,
            inner_size,
            size: inner_size * 5,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let grid = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect::<Vec<u8>>())
        .collect::<Vec<Vec<u8>>>();

    let a: u32 = dijkstra_len(&grid).unwrap();
    println!("A: {}", a);

    let ext_grid = ExtendedGrid::new(grid);
    let b: u32 = dijkstra_len(&ext_grid).unwrap();
    println!("B: {}", b);

    Ok(())
}

fn adj(x: usize, y: usize, size: usize) -> impl Iterator<Item = (usize, usize)> {
    let mut acc: Vec<(usize, usize)> = vec![];
    match x {
        0 => acc.push((1, y)),
        n if n == size - 1 => acc.push((n - 1, y)),
        n => {
            acc.push((n + 1, y));
            acc.push((n - 1, y));
        }
    }
    match y {
        0 => acc.push((x, 1)),
        n if n == size - 1 => acc.push((x, n - 1)),
        n => {
            acc.push((x, n + 1));
            acc.push((x, n - 1));
        }
    }
    acc.into_iter()
}

fn dijkstra_len<T, U>(grid: &dyn Grid2D<T>) -> Option<U>
where
    U: Default + Clone + Copy + From<T> + Add<Output = U> + PartialOrd + Ord,
{
    let mut sdv = vec![vec![None; grid.size()]; grid.size()];
    sdv[0][0] = Some(U::default());
    let mut visited = vec![vec![false; grid.size()]; grid.size()];
    let mut queue = vec![((0usize, 0usize), U::default())];
    while let Some(((cx, cy), _)) = queue.pop() {
        let mut flag = false;
        for (sx, sy) in adj(cx, cy, grid.size()).filter(|(x, y)| !visited[*x][*y]) {
            let dist = sdv[cx][cy].unwrap() + grid.get(sx, sy).into();
            if let Some(prev) = sdv[sx][sy] {
                if dist < prev {
                    *queue
                        .iter_mut()
                        .find(|((x, y), _)| *x == sx && *y == sy)
                        .unwrap() = ((sx, sy), dist);
                    sdv[sx][sy] = Some(dist);
                    flag = true;
                }
            } else {
                queue.push(((sx, sy), dist));
                sdv[sx][sy] = Some(dist);
                flag = true;
            }
        }
        visited[cx][cy] = true;
        if flag {
            queue.sort_by(|(_, a), (_, b)| b.cmp(a));
        }
    }
    sdv[grid.size() - 1][grid.size() - 1]
}
