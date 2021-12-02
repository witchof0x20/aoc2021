use std::io::{self, BufRead};

enum WindowState {
    E0,
    E1,
    E2,
    E3,
    E4,
}
fn main() -> Result<(), io::Error> {
    use WindowState::*;
    let (_, _, part1_result, part2_result) = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.parse())
        .flatten()
        .fold(
            ([0, 0, 0, 0], E0, 0usize, 0usize),
            |([a, b, c, d], win_state, count1, count2), x: u64| match win_state {
                E0 => ([x, 0, 0, 0], E1, 0, 0),
                E1 => ([a, x, 0, 0], E2, (x > a).into(), 0),
                E2 => ([a, b, x, 0], E3, count1 + usize::from(x > b), 0),
                E3 => (
                    [a, b, c, x],
                    E4,
                    count1 + usize::from(x > c),
                    (x > a).into(),
                ),
                E4 => (
                    [b, c, d, x],
                    E4,
                    count1 + usize::from(x > d),
                    count2 + usize::from(x > b),
                ),
            },
        );

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}
