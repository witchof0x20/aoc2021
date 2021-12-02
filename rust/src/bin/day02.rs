use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let (p1_horiz, p1_depth, p2_horiz, p2_depth, _) = io::stdin().lock().lines().fold(
        (0i64, 0i64, 0i64, 0i64, 0i64),
        |(p1_horiz, p1_depth, p2_horiz, p2_depth, aim), line| {
            let line = line.unwrap();
            let mut toks = line.split(' ');
            let dir = toks.next().unwrap();
            let x: i64 = toks.next().unwrap().parse().unwrap();
            match dir {
                "forward" => (
                    p1_horiz + x,
                    p1_depth,
                    p2_horiz + x,
                    p2_depth + aim * x,
                    aim,
                ),
                "down" => (p1_horiz, p1_depth + x, p2_horiz, p2_depth, aim + x),
                "up" => (p1_horiz, p1_depth - x, p2_horiz, p2_depth, aim - x),
                _ => unimplemented!(),
            }
        },
    );
    println!("Part 1 result: {}", p1_horiz * p1_depth);
    println!("Part 2 result: {}", p2_horiz * p2_depth);
    Ok(())
}
