fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::fs::read_to_string("input.txt")?
        .lines()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    let mut sum = 0u32;
    let mut acc = vec![];
    for (line, i) in input.iter_mut().zip(0usize..) {
        acc.push(vec![]);
        for c in line.chars() {
            if is_opening(c) {
                acc[i].push(c);
            } else {
                let opening = acc[i].pop().unwrap();
                if char_completion(opening) != c {
                    sum += char_value_a(c);
                    line.truncate(0);
                    break;
                }
            }
        }
    }

    println!("A: {}", sum);

    let incomplete = input
        .into_iter()
        .zip(acc.into_iter())
        .filter(|(s, _)| !s.is_empty())
        .map(|(_, acc)| acc);

    let mut sum_vec = vec![];
    for v in incomplete {
        sum_vec.push(complete_vec(v));
    }
    sum_vec.sort_unstable();

    println!("B: {}", sum_vec[sum_vec.len() / 2]);
    Ok(())
}

fn char_value_a(c: char) -> u32 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => unreachable!(),
    }
}

fn char_value_b(c: char) -> u32 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => unreachable!(),
    }
}

fn is_opening(c: char) -> bool {
    matches!(c, '{' | '[' | '(' | '<')
}

fn char_completion(c: char) -> char {
    match c {
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

fn complete_vec(mut v: Vec<char>) -> usize {
    let mut sum = 0usize;
    while let Some(c) = v.pop() {
        sum *= 5;
        sum += char_value_b(char_completion(c)) as usize;
    }
    sum
}
