const NEWLINE_CHAR: char = '\n';

#[derive(Debug)]
struct BitMatrix {
    content: Vec<Vec<u8>>,
    width: usize,
}

impl From<Vec<u8>> for BitMatrix {
    fn from(v: Vec<u8>) -> Self {
        let mut width = None;
        let content = v
            .as_slice()
            .split(|c| *c == NEWLINE_CHAR as u8)
            .filter_map(|slice| {
                let slice = slice
                    .iter()
                    .map(|x| match *x as char {
                        '1' => 1u8,
                        '0' => 0u8,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<u8>>();
                match width {
                    Some(s) if slice.len() != s => return None,
                    None => width = Some(slice.len()),
                    _ => (),
                };
                Some(slice)
            })
            .collect::<Vec<Vec<u8>>>();
        BitMatrix {
            content,
            width: width.unwrap(),
        }
    }
}

fn bit_average(input: &[Vec<u8>], col: usize, parity: bool) -> u8 {
    let mut sum = 0;
    input
        .iter()
        .for_each(|x| if x[col] == 0 { sum -= 1 } else { sum += 1 });
    if parity && sum == 0 {
        1
    } else {
        sum.clamp(0, 1) as u8
    }
}

fn bit_average_neg(input: &[Vec<u8>], col: usize, parity: bool) -> u8 {
    1 & !bit_average(input, col, parity)
}

fn bit_slice_to_u64(input: &[u8]) -> u64 {
    input
        .iter()
        .map(|x| *x as u64)
        .reduce(|acc, x| (acc << 1) | x)
        .unwrap()
}

fn oxygen_rating(input: &BitMatrix) -> u64 {
    let mut filtered = input.content.clone();
    let mut current = 0usize;
    while filtered.len() > 1 {
        let w = bit_average(&filtered, current, true);
        filtered = filtered
            .into_iter()
            .filter(|x| x[current] == w)
            .collect::<Vec<Vec<u8>>>();
        current += 1;
    }
    bit_slice_to_u64(&filtered[0])
}

fn co2_scrubber_rating(input: &BitMatrix) -> u64 {
    let mut filtered = input.content.clone();
    let mut current = 0usize;
    while filtered.len() > 1 {
        let w = bit_average_neg(&filtered, current, true);
        filtered = filtered
            .into_iter()
            .filter(|x| x[current] == w)
            .collect::<Vec<Vec<u8>>>();
        current += 1;
    }
    bit_slice_to_u64(&filtered[0])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = BitMatrix::from(std::fs::read("input.txt")?);

    // First Part

    let gamma = bit_slice_to_u64(
        &(0..input.width)
            .map(|i| bit_average(&input.content, i, false))
            .collect::<Vec<u8>>(),
    );
    let epsilon = bit_slice_to_u64(
        &(0..input.width)
            .map(|i| bit_average_neg(&input.content, i, false))
            .collect::<Vec<u8>>(),
    );

    println!("Power consumption: {}", gamma * epsilon);

    // Second Part

    let oxygen = oxygen_rating(&input);
    let co2_scrubber = co2_scrubber_rating(&input);

    println!("Life rating: {}", oxygen * co2_scrubber);

    Ok(())
}
