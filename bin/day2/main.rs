use itertools::Itertools;
fn part1(input: &str) -> u32 {
    let result: u32 = input.lines()
    .filter_map(|line| {
        line.trim()
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .minmax()
            .into_option()
    })
    .map(|(min, max)| max - min)
    .sum();
result
}
fn part2(input: &str) -> u32 {
    let result: u32 = input.lines()
    .map(|line| {
        line.trim()
            .split_whitespace()
            .filter_map(|n| n.parse::<u32>().ok())
            .permutations(2)
            .find(|pair| pair[0] % pair[1] == 0)
            .map_or(0, |perm| perm[0] / perm[1])
    })
    .sum();
    result
}
fn main() {
    let input = include_str!("input2.txt");
    println!("{}, {}", part1(input), part2(input));
}