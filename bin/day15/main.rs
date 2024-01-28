
const D: u64 = 2147483647;

#[derive(Debug, Clone)]
struct Gen {
    v: u64,
    ftr: u64,
    mlp: u64,
}

impl Gen {
    fn new(v: u64, ftr: u64, mlp: u64) -> Self {
        Self { v, ftr, mlp }
    }

    fn next(&mut self) -> u64 {
        loop {
            self.v = (self.v * self.ftr) % D;
            if self.v % self.mlp == 0 {
                return self.v;
            }
        }
    }
}

#[derive(Debug, Clone)]
struct Judge {
    gns: [Gen; 2],
    rds: u32,
}

impl Judge {
    fn new(gns: [Gen; 2], rds: u32) -> Self {
        Self { gns, rds }
    }

    fn final_count(&mut self) -> u64 {
        let mut sum = 0;
        for _ in 0..self.rds {
            let v1 = self.gns[0].next();
            let v2 = self.gns[1].next();
            if v1 & 0xFFFF == v2 & 0xFFFF {
                sum += 1;
            }
        }
        sum
    }
}

fn part1(st: (u64, u64)) -> u64 {
    let rounds = 40_000_000;
    let g1 = Gen::new(st.0, 16807, 1);
    let g2 = Gen::new(st.1, 48271, 1);
    let mut judge = Judge::new([g1, g2], rounds);

    judge.final_count()
}

fn part2(st: (u64, u64)) -> u64 {
    let rounds = 5_000_000;
    let g1 = Gen::new(st.0, 16807, 4);
    let g2 = Gen::new(st.1, 48271, 8);
    let mut judge = Judge::new([g1, g2], rounds);

    judge.final_count()
}

fn main() {
    println!("{}", part1((783, 325)));
}
