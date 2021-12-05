use std::collections::HashSet;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    fn parse(input: &str) -> Result<Point, Box<dyn std::error::Error>> {
        let first = input.find(',').unwrap();
        Ok(Point {
            x: input[..first].parse::<usize>()?,
            y: input[first + ','.len_utf8()..].parse::<usize>()?,
        })
    }

    fn abs_diff(self, rhs: Point) -> Point {
        Point {
            x: self.x.checked_sub(rhs.x).unwrap_or_else(|| rhs.x - self.x),
            y: self.y.checked_sub(rhs.y).unwrap_or_else(|| rhs.y - self.y),
        }
    }

    fn max_distance(self, rhs: Point) -> usize {
        let diff = self.abs_diff(rhs);
        usize::max(diff.x, diff.y)
    }
}

struct Coords {
    lines: Vec<(Point, Point)>,
    boundaries: Point,
}

impl Coords {
    fn init() -> Coords {
        Coords {
            lines: vec![],
            boundaries: Point::new(0, 0),
        }
    }

    fn parse(input: &str) -> Result<Coords, Box<dyn std::error::Error>> {
        let splitted = input.lines().filter(|x| !x.is_empty());
        let mut coords = Coords::init();
        for line in splitted {
            let mut subsplit = line.split(" -> ");
            let (n1, n2) = (
                Point::parse(subsplit.next().unwrap())?,
                Point::parse(subsplit.next().unwrap())?,
            );
            let (mx, my) = (usize::max(n1.x, n2.x), usize::max(n1.y, n2.y));
            if coords.boundaries.x <= mx {
                coords.boundaries.x = mx + 1;
            }
            if coords.boundaries.y <= my {
                coords.boundaries.y = my + 1;
            }
            coords.lines.push((n1, n2));
        }
        Ok(coords)
    }
}

struct Grid {
    cache: HashSet<Point>,
}

fn calculate_line_indexes(start: Point, end: Point) -> Vec<Point> {
    let max_dist = start.max_distance(end);

    let x_iter = match start.x.checked_sub(end.x) {
        Some(n) if n == 0 => vec![start.x; max_dist + 1],
        Some(_) => (end.x..=start.x).collect::<Vec<usize>>(),
        None => {
            let mut to_rev = (start.x..=end.x).collect::<Vec<usize>>();
            to_rev.reverse();
            to_rev
        }
    };

    let y_iter = match start.y.checked_sub(end.y) {
        Some(n) if n == 0 => vec![start.y; max_dist + 1],
        Some(_) => (end.y..=start.y).collect::<Vec<usize>>(),
        None => {
            let mut to_rev = (start.y..=end.y).collect::<Vec<usize>>();
            to_rev.reverse();
            to_rev
        }
    };

    x_iter
        .into_iter()
        .zip(y_iter)
        .map(|(x, y)| Point::new(x, y))
        .collect::<Vec<Point>>()
}

fn is_diagonal(start: &Point, end: &Point) -> bool {
    let diff = start.abs_diff(*end);
    diff.x == diff.y
}

impl Grid {
    fn populate(coords: Coords, diagonal: bool) -> Grid {
        let mut cells = vec![vec![0u16; coords.boundaries.y]; coords.boundaries.x];
        let mut cache = HashSet::new();
        coords
            .lines
            .into_iter()
            .filter(|(start, end)| {
                (start.x == end.x || start.y == end.y) || (diagonal && is_diagonal(start, end))
            })
            .map(|(start, end)| calculate_line_indexes(start, end))
            .for_each(|points| {
                points.iter().for_each(|p| {
                    cells[p.x][p.y] += 1;
                    if cells[p.x][p.y] == 2 {
                        cache.insert(*p);
                    }
                })
            });
        Grid { cache }
    }

    fn count_overlaps(&self) -> usize {
        self.cache.len()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = String::from_utf8(std::fs::read("input.txt")?)?;
    let coords = Coords::parse(input.as_str())?;
    let grid = Grid::populate(coords, false);
    println!("P1: There are {} h/v overlaps.", grid.count_overlaps());

    let coords = Coords::parse(input.as_str())?;
    let grid = Grid::populate(coords, true);
    println!("P2: There are {} h/v/d overlaps.", grid.count_overlaps());
    Ok(())
}
