use std::io::{self, BufRead};

enum WindowState {
    L0,
    L1,
    L2,
    L3,
    L4,
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
            ([0, 0, 0, 0], L0, 0, 0),
            |([a, b, c, d], length, count1, count2), x: u64| match length {
                // Window is empty, append the first element
                L0 => ([x, 0, 0, 0], L1, 0, 0),
                // Window has 1 element, append the second and compare
                L1 => ([a, x, 0, 0], L2, (x > a).into(), 0),
                // Window has 2 elements, append the third and compare
                L2 => ([a, b, x, 0], L3, count1 + usize::from(x > b), 0),
                // Window has 3 elements, append the fourth and compare for
                // both problems
                // We compare each element with the element 3 backwards because
                // b + c + d > a + b + c is equivalent to
                // d > a
                L3 => (
                    [a, b, c, x],
                    L4,
                    count1 + usize::from(x > c),
                    (x > a).into(),
                ),
                // Window has 4 elements. Shift left, compare for both problems
                // This is the terminal state, and will continue for all future
                // elements
                L4 => (
                    [b, c, d, x],
                    L4,
                    count1 + usize::from(x > d),
                    count2 + usize::from(x > b),
                ),
            },
        );

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}
