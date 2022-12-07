use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, fs};

const START_LOW: u8 = b'a' - 1;
const START_HIGH: u8 = b'A' - 1;

fn main() -> Result<()> {
    let first = fs::read_to_string("./src/bin/input3.prod")?
        .lines()
        .flat_map(|line| {
            let half = line.len() / 2;
            let (first, second) = line.split_at(half);
            let first = first.chars().collect::<HashSet<_>>();
            let second = second.chars().collect::<HashSet<_>>().into_iter();
            return second.filter(move |c| first.contains(c)).map(|c| {
                if c.is_ascii_lowercase() {
                    return c as u8 - START_LOW;
                }
                return c as u8 - START_HIGH + 26;
            });
        })
        .map(|c| c as u32)
        .sum::<u32>();

    println!("Part 1: {}", first);

    let second = fs::read_to_string("./src/bin/input3.prod")?
        .lines()
        .tuples()
        .flat_map(|(a, b, c)| {
            let first = a.chars().collect::<HashSet<_>>();
            let second = b.chars().collect::<HashSet<_>>();
            let third = c.chars().collect::<HashSet<_>>().into_iter();
            return third
                .filter(move |c| first.contains(c))
                .filter(move |c| second.contains(c))
                .map(|c| {
                    if c.is_ascii_lowercase() {
                        return c as u8 - START_LOW;
                    }
                    return c as u8 - START_HIGH + 26;
                });
        })
        .map(|c| c as u32)
        .sum::<u32>();

    println!("Part 2: {}", second);

    return Ok(());
}
