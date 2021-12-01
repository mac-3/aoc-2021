use std::error::Error;
use std::fs;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn Error>> {
    let (_, v) = fs::read("input.txt")?
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|n| n.parse::<u32>().ok())
        .fold((None, 0u32), |(last, count), curr| match last {
            Some(last) => (Some(curr), if last < curr { count + 1 } else { count }),
            None => (Some(curr), count),
        });
    println!("{}", v);
    Ok(())
}
