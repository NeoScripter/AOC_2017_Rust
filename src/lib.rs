#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused)]

use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
struct Spinlock (Vec<u32>);

impl Spinlock {
    fn new() -> Self {
        Self (vec![1, 0])
    }
    fn process(&mut self, step: usize) {
        for n in 2..=2017 {
            let len = self.0.len();
            self.0.rotate_left(step % len);
            self.0.insert(1, n);
            self.0.rotate_left(1);
        }
    }
}
fn part1() -> u32 {
    let step = 3;
    let mut spin = Spinlock::new();
    spin.process(step);
    spin.0[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(638, part1());
    }
}