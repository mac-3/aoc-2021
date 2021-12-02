use std::error::Error;
use std::fs;
use std::io::BufRead;

const WINDOW_SIZE: usize = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let (_, count) = fs::read("input.txt")?
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>()
        .as_slice()
        .windows(WINDOW_SIZE)
        .map(|w| w.iter().cloned().reduce(|a, b| a + b))
        .fold((None, 0u32), |(last, count), curr| match last {
            Some(last) => (Some(curr), if last < curr { count + 1 } else { count }),
            None => (Some(curr), count),
        });
    println!("{}", count);
    Ok(())
}
