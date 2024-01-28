use std::collections::{HashSet, HashMap};
#[derive(Debug, Clone)]
struct Blocks(Vec<u32>);

impl Blocks {
    fn find_max(&self) -> (u32, usize) {
        let max_el = *self.0.iter().max().unwrap();
        let idx = self.0.iter().position(|&x| x == max_el).unwrap();
        (max_el, idx)
    }
    
    fn redistribute(&mut self, max_el: u32, mut idx: usize) {
        self.0[idx] = 0;
        let len = self.0.len();
        let mut blocks = max_el;
        idx += 1;
        
        while blocks > 0 {
            self.0[idx % len] += 1;
            idx += 1;
            blocks -= 1;
        }
    }
}

fn parse(input: &str) -> Blocks {
    let blocks = input.split_whitespace().filter_map(|x| x.parse::<u32>().ok()).collect();
    Blocks(blocks)
}

fn part1(input: &str) -> usize {
    let mut blocks = parse(input);
    let mut seen = HashSet::new();
    seen.insert(blocks.0.clone());
    loop {
        let (max_el, idx) = blocks.find_max();
        blocks.redistribute(max_el, idx);
        let cycle = blocks.0.clone();
        if !seen.insert(cycle) {
            return seen.len();
        }
    }
}
fn part2(input: &str) -> usize {
    let mut blocks = parse(input);
    let mut seen = HashMap::new();
    seen.insert(blocks.0.clone(), 1);
    loop {
        let (max_el, idx) = blocks.find_max();
        blocks.redistribute(max_el, idx);
        let cycle = blocks.0.clone();
        *seen.entry(cycle).or_insert(0) += 1;
        if seen.values().any(|&v| v == 3) {
            return seen.into_iter().filter(|(_, value)| *value >= 2).count();
        }
    }
}

fn main() {
    let input = include_str!("input6.txt");
    println!("{}, {}", part1(input), part2(input));
}