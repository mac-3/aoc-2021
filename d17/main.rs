fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string("input.txt")?;
    let ((minx, maxx), (miny, maxy)) = input
        .split_once("x=")
        .unwrap()
        .1
        .split_once(", y=")
        .map(|(xstr, ystr)| {
            let xbounds = xstr
                .split_once("..")
                .map(|(minx, miny)| (minx.parse::<i32>().unwrap(), miny.parse::<i32>().unwrap()))
                .unwrap();
            let ybounds = ystr
                .split_once("..")
                .map(|(minx, miny)| (minx.parse::<i32>().unwrap(), miny.parse::<i32>().unwrap()))
                .unwrap();
            (xbounds, ybounds)
        })
        .unwrap();

    println!("A: {}", gauss_sum(max_vel_y(miny..=maxy)));

    let mut count = 0usize;
    for x in min_vel_x(minx..=maxx)..=max_vel_x(minx..=maxx) {
        for y in min_vel_y(miny..=maxy)..=max_vel_y(miny..=maxy) {
            if simulate(x, y, minx..=maxx, miny..=maxy) {
                count += 1;
            }
        }
    }
    println!("B: {}", count);

    Ok(())
}

fn max_vel_y(interval: std::ops::RangeInclusive<i32>) -> i32 {
    interval.start().abs() - 1
}

fn min_vel_y(interval: std::ops::RangeInclusive<i32>) -> i32 {
    *interval.start()
}

fn max_vel_x(interval: std::ops::RangeInclusive<i32>) -> i32 {
    *interval.end()
}

fn min_vel_x(interval: std::ops::RangeInclusive<i32>) -> i32 {
    ((f32::sqrt((1 + (8 * interval.start())) as f32) - 1.0) / 2.0).ceil() as i32
}

fn gauss_sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn simulate(
    mut vx: i32,
    mut vy: i32,
    intx: std::ops::RangeInclusive<i32>,
    inty: std::ops::RangeInclusive<i32>,
) -> bool {
    let (mut px, mut py) = (0, 0);
    while py >= *inty.start() {
        if py <= *inty.end() && (px >= *intx.start() && px <= *intx.end()) {
            return true;
        }
        px += vx;
        py += vy;
        vx -= vx.signum();
        vy -= 1;
    }
    false
}
