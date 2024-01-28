use itertools::Itertools;

fn part1(input: &str) -> u32 {
    let numbers: Vec<u32> = input.chars().filter_map(|c| c.to_digit(10)).collect();
    numbers.iter().circular_tuple_windows().fold(0, |acc, (&x, &y)| if x == y { acc + x } else { acc })
}
fn part2(input: &str) -> u32 {
    let (left, right) = input.split_at(input.len() / 2);
    let result: u32 = left.bytes().zip(right.bytes())
    .filter_map(|(a, b)| (a == b)
    .then(|| (a - b'0') as u32 * 2))
    .sum();
    result
}
fn main() {
    let input = include_str!("input1.txt");
    println!("{}, {}", part1(input), part2(input));
}