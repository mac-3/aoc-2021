pub fn part_a(input: &str) -> usize {
    let mut acc = [0u8; 10];
    input
        .lines()
        .map(|s| {
            let sep = s.find('|').expect("Line must contain separator.");
            s[sep + '|'.len_utf8()..].split(' ')
        })
        .for_each(|s| {
            s.for_each(|s| match s.len() {
                2 => acc[1] += 1,
                3 => acc[7] += 1,
                4 => acc[4] += 1,
                7 => acc[8] += 1,
                _ => (),
            });
        });
    acc.iter().map(|u| *u as usize).sum::<usize>()
}
