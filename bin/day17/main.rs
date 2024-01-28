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
    let step = 345;
    let mut spin = Spinlock::new();
    spin.process(step);
    spin.0[1]
}

fn part2() -> u32 {
    let step = 345;
    let mut pos = 1;
    let mut next_value = 1;
    let rounds = 50_000_000;

    for i in 2..=rounds {
        pos = (pos + step) % i;
        if pos == 0 {
            next_value = i;
        }
        pos += 1;
    }
    next_value
}

fn main() {
    println!("{}", part2());
}