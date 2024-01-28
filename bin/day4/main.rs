use itertools::Itertools;

fn part1(input: &str) -> usize {
    input.lines().filter(|line| line.trim().split_whitespace().all_unique()).count()
}
fn part2(input: &str) -> usize {
    input.lines()
    .filter(|line| line.trim()
    .split_whitespace()
    .map(|word| word.chars()
    .sorted_unstable()
    .collect::<String>())
    .all_unique())
    .count()
}
fn main() {
    let input = include_str!("input4.txt");
    println!("{}, {}", part1(input), part2(input));
}