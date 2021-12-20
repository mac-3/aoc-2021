const ITERATIONS: usize = 50;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (enh, mut img) = std::fs::read_to_string("input.txt")?
        .split_once("\n\n")
        .map(|(enh, img)| {
            (
                enh.chars().map(|c| matches!(c, '#')).collect::<Vec<bool>>(),
                img.lines()
                    .map(|x| x.chars().map(|c| matches!(c, '#')).collect::<Vec<bool>>())
                    .collect::<Vec<Vec<bool>>>(),
            )
        })
        .unwrap();
    let mut outside = false;
    enhance_assign(&enh, &mut img, &mut outside, 2);
    let twice = img.clone();
    enhance_assign(&enh, &mut img, &mut outside, ITERATIONS - 2);
    println!("A: {}", count_lit(&twice));
    println!("B: {}", count_lit(&img));
    Ok(())
}

fn count_lit(img: &[Vec<bool>]) -> usize {
    img.iter()
        .map(|row| row.iter().filter(|col| **col).count())
        .sum::<usize>()
}

fn enhance_assign(enh: &[bool], img: &mut Vec<Vec<bool>>, outside: &mut bool, iter: usize) {
    (0..iter).for_each(|_| {
        let (img_new, outside_new) = enhance(img, enh, *outside);
        *img = img_new;
        *outside = outside_new;
    })
}

pub fn enhance(image: &[Vec<bool>], enh: &[bool], outside: bool) -> (Vec<Vec<bool>>, bool) {
    let mut acc = vec![];
    for x in -1..=image.len() as i32 {
        let mut row = vec![];
        for y in -1..=image.len() as i32 {
            row.push(enh[enhancement_index(image, x as i32, y as i32, outside)]);
        }
        acc.push(row);
    }
    (acc, if enh[0] { !outside } else { outside })
}

fn enhancement_index(image: &[Vec<bool>], x: i32, y: i32, lighted: bool) -> usize {
    let mut acc = [false; 9];
    let mut count = 0usize;
    for x in (x - 1)..=(x + 1) {
        for y in (y - 1)..=(y + 1) {
            if x < 0 || x >= image.len() as i32 || y < 0 || y >= image.len() as i32 {
                acc[count] = lighted;
            } else {
                acc[count] = image[x as usize][y as usize];
            }
            count += 1;
        }
    }
    acc.into_iter()
        .fold(0usize, |acc, x| (acc << 1) + if x { 1 } else { 0 })
}
