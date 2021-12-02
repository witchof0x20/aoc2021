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
                if let Some(dir) = toks.next() {
                    if let Some(x) = toks.next() {
                        if let Ok(x) = x.parse::<i64>() {
                            Some(match dir {
                                "forward" => (horiz + x, p1_depth, p2_depth + aim * x, aim),
                                "down" => (horiz, p1_depth + x, p2_depth, aim + x),
                                "up" => (horiz, p1_depth - x, p2_depth, aim - x),
                                _ => unimplemented!(),
                            })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        )
        .expect("Error");
    println!("Part 1 result: {}", horiz * p1_depth);
    println!("Part 2 result: {}", horiz * p2_depth);
    Ok(())
}
