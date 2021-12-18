use std::{ops::Add, str::FromStr};

#[derive(Debug)]
pub struct Expr {
    elems: Vec<u32>,
    groups: Vec<Vec<usize>>,
    gid: usize,
}

impl FromStr for Expr {
    type Err = std::fmt::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = &mut &*s;
        let mut elems = vec![];
        let mut groups = vec![];
        let mut groups_stack = vec![];
        let mut gid = 0usize;
        while let Some(first_char) = bump_char(s) {
            match first_char {
                '[' => {
                    groups_stack.push(gid);
                    gid += 1;
                }
                ']' => {
                    groups_stack.pop().unwrap();
                }
                c @ '0'..='9' => {
                    let end = s.find(|c: char| !c.is_numeric()).unwrap_or_else(|| s.len());
                    let n = if end > 0 {
                        let mut n = s[..end].parse::<u32>().unwrap();
                        n += (((c as u8) - b'0') as u32) * 10u32.pow(end as u32);
                        n
                    } else {
                        (c as u8 - b'0') as u32
                    };
                    elems.push(n);
                    groups.push(groups_stack.clone());
                    *s = &s[end..];
                }
                _ => (),
            }
        }
        Ok(Expr { elems, groups, gid })
    }
}

fn bump_char(input: &mut &str) -> Option<char> {
    if let Some(c) = input.chars().next() {
        *input = &input[c.len_utf8()..];
        Some(c)
    } else {
        None
    }
}

impl Add for Expr {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        let mut rhs = rhs;
        rhs.reduce();
        self.elems.append(&mut rhs.elems);
        rhs.groups
            .iter_mut()
            .for_each(|v| v.iter_mut().for_each(|x| *x += self.gid));
        self.gid += rhs.gid;
        self.groups.append(&mut rhs.groups);
        self.groups.iter_mut().for_each(|v| v.insert(0, self.gid));
        self.gid += 1;
        self.reduce();
        self
    }
}

impl Expr {
    pub fn reduce(&mut self) -> usize {
        let mut count = 0usize;
        'main: loop {
            for (_, i) in self.elems.iter().zip(0usize..) {
                if self.groups[i].len() > 4
                    && self.groups[i].last().unwrap() == self.groups[i + 1].last().unwrap()
                {
                    self.explode(i);
                    count += 1;
                    continue 'main;
                }
            }
            for (e, i) in self.elems.iter().zip(0usize..) {
                if *e >= 10 {
                    self.split(i);
                    count += 1;
                    continue 'main;
                }
            }
            break;
        }
        count
    }

    fn explode(&mut self, i: usize) {
        let (sx, dx) = (self.elems.remove(i), self.elems[i]);
        self.groups.remove(i);
        self.groups[i].pop().unwrap();
        self.elems[i] = 0;
        if i > 0 {
            self.elems[i - 1] += sx;
        }
        if i < self.elems.len() - 1 {
            self.elems[i + 1] += dx
        };
    }

    fn split(&mut self, i: usize) {
        let n = self.elems[i];
        let (n1, n2) = (n / 2, (n / 2) + (n % 2));
        self.elems[i] = n2;
        self.elems.insert(i, n1);
        self.groups[i].push(self.gid);
        self.groups.insert(i, self.groups[i].clone());
        self.gid += 1;
    }

    pub fn resolve(mut self) -> u32 {
        while self.elems.len() > 2 {
            for i in 0..self.elems.len() - 1 {
                if self.groups[i].last().unwrap() == self.groups[i + 1].last().unwrap() {
                    self.elems[i] = (3 * self.elems[i]) + (2 * self.elems[i + 1]);
                    self.elems.remove(i + 1);
                    self.groups.remove(i + 1);
                    self.groups[i].pop().unwrap();
                    break;
                }
            }
        }
        (self.elems[0] * 3) + (self.elems[1] * 2)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let expr = input
        .lines()
        .map(|line| line.parse::<Expr>().unwrap())
        .reduce(|acc, x| acc + x)
        .unwrap();
    println!("A: {}", expr.resolve());

    let lines = input
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let mut max_magnitude = None;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i != j {
                let curr = (lines[i].parse::<Expr>()? + lines[j].parse::<Expr>()?).resolve();
                match max_magnitude {
                    Some(ref mut max) => {
                        if curr > *max {
                            *max = curr
                        }
                    }
                    None => max_magnitude = Some(curr),
                }
            }
        }
    }
    println!("B: {}", max_magnitude.unwrap());

    Ok(())
}
