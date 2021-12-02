use std::io::{self, BufRead};
use std::str::FromStr;

fn count_ups(it: impl Iterator<Item = impl PartialOrd + Copy>) -> usize {
    it.fold((None, 0), |(last, count), x| {
        last.map(|last| (Some(x), if x > last { count + 1 } else { count }))
            .unwrap_or_else(|| (Some(x), 0))
    })
    .1
}
fn main() -> Result<(), io::Error> {
    let input: Vec<u64> = io::stdin()
        .lock()
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let part1_result = count_ups(input.iter());
    let part2_result = count_ups(input.windows(3).map(|w| w.iter().sum::<u64>()));
    println!("Part 1: {}", part1_result);
    println!("Part 2: {}", part2_result);
    Ok(())
}
