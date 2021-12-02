use std::io::{self, BufRead};

fn main() -> Result<(), io::Error> {
    let input: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .flatten()
        .map(|line| line.parse())
        .flatten()
        .collect();

    let (part1_result, part2_result) = input
        .iter()
        .zip(input.iter().skip(1))
        .zip(input.iter().skip(3).chain(&[0, 0]))
        .fold((0, 0), |(count1, count2), ((a, b), c)| {
            (count1 + usize::from(b > a), count2 + usize::from(c > a))
        });

    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);

    Ok(())
}
