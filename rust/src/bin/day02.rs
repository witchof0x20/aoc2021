use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let (horiz, p1_depth, p2_depth, _) = io::stdin()
        .lock()
        .lines()
        .flatten()
        .try_fold(
            (0i64, 0i64, 0i64, 0i64),
            |(horiz, p1_depth, p2_depth, aim), line| {
                let mut toks = line.split(' ');
                toks.next().zip(toks.next()).and_then(|(dir, x)| {
                    x.parse().ok().and_then(|x: i64| match dir {
                        "forward" => Some((horiz + x, p1_depth, p2_depth + aim * x, aim)),
                        "down" => Some((horiz, p1_depth + x, p2_depth, aim + x)),
                        "up" => Some((horiz, p1_depth - x, p2_depth, aim - x)),
                        _ => None,
                    })
                })
            },
        )
        .unwrap();
    println!("Part 1 result: {}", horiz * p1_depth);
    println!("Part 2 result: {}", horiz * p2_depth);
    Ok(())
}
