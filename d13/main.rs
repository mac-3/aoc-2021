use std::collections::HashSet;

#[derive(Debug)]
struct Grid {
    dots: HashSet<(usize, usize)>,
    max: (usize, usize),
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut display = vec![vec!['.'; self.max.0 + 1]; self.max.1 + 1];
        self.dots.iter().for_each(|(x, y)| display[*y][*x] = '#');
        for i in 0..=self.max.1 {
            for j in 0..=self.max.0 {
                write!(f, "{}", display[i][j])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Grid {
    fn parse(input: &str) -> Self {
        let mut max = (0usize, 0usize);
        let dots = input
            .lines()
            .map(|line| {
                let mut splitted = line.split(',');
                let x = splitted.next().unwrap().parse::<usize>().unwrap();
                if x > max.0 {
                    max.0 = x
                }
                let y = splitted.next().unwrap().parse::<usize>().unwrap();
                if y > max.1 {
                    max.1 = y
                }

                (x, y)
            })
            .collect::<HashSet<(usize, usize)>>();
        Grid { dots, max }
    }
}

impl Grid {
    fn fold(&mut self, fold: FoldInstruction) {
        let dots = std::mem::take(&mut self.dots);
        let mut acc = HashSet::with_capacity(dots.capacity());
        match fold {
            FoldInstruction::Left(u) => {
                for (x, y) in dots {
                    let x = if x > u { 2 * u - x } else { x };
                    acc.insert((x, y));
                }
                self.max.0 = u - 1;
            }
            FoldInstruction::Up(u) => {
                for (x, y) in dots {
                    let y = if y > u { 2 * u - y } else { y };
                    acc.insert((x, y));
                }
                self.max.1 = u - 1;
            }
        }
        let _ = std::mem::replace(&mut self.dots, acc);
    }
}

#[derive(Debug)]
enum FoldInstruction {
    Left(usize),
    Up(usize),
}

impl FoldInstruction {
    fn parse(input: &str) -> Self {
        let mut splitted = input["fold along ".len()..].trim().split('=');
        match splitted.next().unwrap() {
            "x" => FoldInstruction::Left(splitted.next().unwrap().parse::<usize>().unwrap()),
            "y" => FoldInstruction::Up(splitted.next().unwrap().parse::<usize>().unwrap()),
            _ => unreachable!(),
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let section_div = input.find("\n\n").unwrap();
    let mut grid = Grid::parse(&input[..section_div]);
    let folds = input[section_div + "\n\n".len()..]
        .lines()
        .map(FoldInstruction::parse);

    let mut count = 0usize;
    let mut first_fold_snapshot = None;
    for fold in folds {
        grid.fold(fold);
        if first_fold_snapshot.is_none() {
            first_fold_snapshot = Some(grid.dots.clone());
        }

        count += 1;
    }

    println!("A: {}\n", first_fold_snapshot.unwrap().len());
    println!("Fold {}:\n\n{}", count, &grid);

    Ok(())
}
