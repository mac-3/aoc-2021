use std::io::BufRead;

#[derive(Default)]
struct Pos {
    x: u32,
    depth: u32,
    aim: u32,
}

enum Command {
    Forward(u32),
    Up(u32),
    Down(u32),
}

impl Command {
    fn parse(input: impl AsRef<str>) -> Option<Command> {
        let mut s = input.as_ref().split_whitespace();
        match s.next().unwrap() {
            "forward" => Some(Command::Forward(s.next()?.parse::<u32>().ok()?)),
            "up" => Some(Command::Up(s.next()?.parse::<u32>().ok()?)),
            "down" => Some(Command::Down(s.next()?.parse::<u32>().ok()?)),
            _ => None,
        }
    }

    fn apply(&self, mut pos: Pos) -> Pos {
        match self {
            Command::Forward(u) => {
                pos.x += *u;
                pos.depth += pos.aim * u;
            }
            Command::Up(u) => pos.aim -= *u,
            Command::Down(u) => pos.aim += *u,
        };
        pos
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read("../assets/input.txt")?
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(Command::parse)
        .fold(Pos::default(), |pos, dir| dir.apply(pos));
    println!("{}", input.x * input.depth);
    Ok(())
}
