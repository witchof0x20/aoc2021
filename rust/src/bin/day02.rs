use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(Debug)]
enum Error {
    IO(io::Error),
    MissingFirstToken,
    InvalidDirection(String),
    MissingSecondToken,
    Parse(usize, <u64 as FromStr>::Err),
}
fn main() -> Result<(), Error> {
    let (horiz, p1_depth, p2_depth, _) = io::stdin()
        .lock()
        .lines()
        .map(|line| line.map_err(Error::IO))
        .enumerate()
        .try_fold(
            (0i64, 0i64, 0i64, 0i64),
            |(horiz, p1_depth, p2_depth, aim), (line_num, line)| {
                line.and_then(|line| {
                    let mut toks = line.split(' ');
                    toks.next()
                        .ok_or_else(|| Error::MissingFirstToken)
                        .and_then(|dir| {
                            toks.next()
                                .map(|x| (dir, x))
                                .ok_or_else(|| Error::MissingSecondToken)
                        })
                        .and_then(|(dir, x)| {
                            x.parse()
                                .map_err(|err| Error::Parse(line_num, err))
                                .and_then(|x: i64| match dir {
                                    "forward" => Ok((horiz + x, p1_depth, p2_depth + aim * x, aim)),
                                    "down" => Ok((horiz, p1_depth + x, p2_depth, aim + x)),
                                    "up" => Ok((horiz, p1_depth - x, p2_depth, aim - x)),
                                    other => Err(Error::InvalidDirection(other.to_owned())),
                                })
                        })
                })
            },
        )
        .unwrap();
    println!("Part 1 result: {}", horiz * p1_depth);
    println!("Part 2 result: {}", horiz * p2_depth);
    Ok(())
}
