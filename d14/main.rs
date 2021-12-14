use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let template_end = input.find('\n').unwrap();
    let template_raw = input[..template_end].chars().collect::<Vec<char>>();
    let mut count = template_raw.iter().fold(HashMap::new(), |mut acc, c| {
        get_or_insert_add(&mut acc, *c, 1);
        acc
    });
    let mut template =
        template_raw
            .windows(2)
            .map(|x| (x[0], x[1]))
            .fold(HashMap::new(), |mut acc, x| {
                get_or_insert_add(&mut acc, x, 1);
                acc
            });
    let map = input[template_end..]
        .lines()
        .filter(|x| !x.trim().is_empty())
        .map(|x| {
            let mut splitted = x.split(" -> ");
            (
                {
                    let mut chars = splitted.next().unwrap().chars();
                    let a = chars.next().unwrap();
                    let b = chars.next().unwrap();

                    (a, b)
                },
                splitted.next().unwrap().chars().next().unwrap(),
            )
        })
        .collect::<HashMap<(char, char), char>>();

    // First 10 iterations
    for _ in 0usize..10 {
        tick(&mut template, &map, &mut count);
    }
    if let Some((min, max)) = get_bounds(&count) {
        println!("A: {}", max - min);
    }

    // Remaining 30 iterations
    for _ in 0usize..30 {
        tick(&mut template, &map, &mut count);
    }
    if let Some((min, max)) = get_bounds(&count) {
        println!("B: {}", max - min);
    }

    Ok(())
}

fn get_bounds(count: &HashMap<char, usize>) -> Option<(usize, usize)> {
    count.values().into_iter().fold(None, |acc, n| match acc {
        Some((min, max)) if *n < min => Some((*n, max)),
        Some((min, max)) if *n > max => Some((min, *n)),
        Some((min, max)) => Some((min, max)),
        None => Some((*n, *n)),
    })
}

fn get_or_insert_add<K, T>(input: &mut HashMap<K, T>, k: K, v: T)
where
    K: Eq + std::hash::Hash + Copy,
    T: std::ops::AddAssign + Copy,
{
    input.entry(k).and_modify(|x| *x += v).or_insert(v);
}

fn tick(
    input: &mut HashMap<(char, char), usize>,
    map: &HashMap<(char, char), char>,
    count: &mut HashMap<char, usize>,
) {
    let changes = input
        .iter()
        .filter_map(|(k, _)| map.get(k).map(|v| ((k.0, k.1), *v)))
        .collect::<Vec<((char, char), char)>>();
    let mut acc = HashMap::new();
    for (old, new) in changes {
        let prev = input.remove(&old).unwrap();
        let e1 = ((old.0, new), prev);
        let e2 = ((new, old.1), prev);
        get_or_insert_add(&mut acc, e1.0, e1.1);
        get_or_insert_add(&mut acc, e2.0, e2.1);
        get_or_insert_add(count, new, prev);
    }
    for (k, v) in acc.into_iter() {
        get_or_insert_add(input, k, v);
    }
}
