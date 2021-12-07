use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = String::from_utf8(std::fs::read("input.txt")?)?
        .split(',')
        .map(|s| s.parse::<u32>())
        .filter_map(Result::ok)
        .collect::<Vec<u32>>();
    let fuel = optimal_fuel(input.as_slice(), |a| a);
    println!("A: {}", fuel);

    let fuel = optimal_fuel(input.as_slice(), sums);
    println!("B: {}", fuel);
    Ok(())
}

fn optimal_fuel(input: &[u32], mut fuel_calc_fn: impl FnMut(u32) -> u32) -> u32 {
    let max = *input.iter().max().expect("input has no elements.");
    (0..max)
        .map(|target| {
            (
                input
                    .iter()
                    .map(|crab| fuel_calc_fn(distance(*crab, target)))
                    .sum::<u32>(),
                target,
            )
        })
        .reduce(|(acc_crab, acc_target), (crab, target)| {
            if acc_crab > crab {
                (crab, target)
            } else {
                (acc_crab, acc_target)
            }
        })
        .map(|(fuel, _)| fuel)
        .unwrap()
}

fn distance(a: u32, b: u32) -> u32 {
    if let Some(r) = a.checked_sub(b) {
        r
    } else {
        b - a
    }
}

fn sums(a: u32) -> u32 {
    (a / 2) * (1 + a) + if a % 2 == 1 { a / 2 + 1 } else { 0 }
}
