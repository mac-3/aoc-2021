use std::error::Error;
use std::fs;
use std::io::BufRead;

const MEASURE_LENGTH: usize = 3;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read("input.txt")?
        .lines()
        .filter_map(|r| r.ok())
        .filter_map(|n| n.parse::<u32>().ok())
        .collect::<Vec<u32>>();

    let mut v = &input[..];
    let (mut last, mut count) = (None, 0u32);

    while v.len() >= MEASURE_LENGTH {
        // SAFETY: It is guaranted by the loop condition to not fail.
        let curr = v[..MEASURE_LENGTH]
            .iter()
            .cloned()
            .reduce(|a, b| a + b)
            .unwrap();
        v = &v[1..];
        last = match last {
            Some(last) => {
                if last < curr {
                    count += 1;
                }
                Some(curr)
            }
            None => Some(curr),
        }
    }
    println!("{}", count);
    Ok(())
}
