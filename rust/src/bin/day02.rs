use std::io::{self, BufRead};

enum Direction {
    Forward,
    Down,
    Up,
}
fn main() -> Result<(), io::Error> {
    let input: Vec<_> = io::stdin()
        .lock()
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut toks = line.split(' ');
            let dir = match toks.next().unwrap() {
                "forward" => Direction::Forward,
                "down" => Direction::Down,
                "up" => Direction::Up,
                _ => unimplemented!(),
            };
            let num: i64 = toks.next().unwrap().parse().unwrap();
            (dir, num)
        })
        .collect();
    let (p1_horiz, p1_depth, p2_horiz, p2_depth, _) = input.iter().fold(
        (0i64, 0i64, 0i64, 0i64, 0i64),
        |(p1_horiz, p1_depth, p2_horiz, p2_depth, aim), (dir, x)| match dir {
            Direction::Forward => (
                p1_horiz + x,
                p1_depth,
                p2_horiz + x,
                p2_depth + aim * x,
                aim,
            ),
            Direction::Down => (p1_horiz, p1_depth + x, p2_horiz, p2_depth, aim + x),
            Direction::Up => (p1_horiz, p1_depth - x, p2_horiz, p2_depth, aim - x),
        },
    );
    println!("Part 1 result: {}", p1_horiz * p1_depth);
    println!("Part 2 result: {}", p2_horiz * p2_depth);
    Ok(())
}
