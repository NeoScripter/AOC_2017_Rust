use std::fmt;

#[derive(Debug, Clone)]
struct Scanner {
    rng: Vec<u8>,
    down: bool,
    pos: usize,
}

impl Scanner {
    fn new(rng: usize) -> Self {
        let mut scr = Self {
            rng: vec![0; rng],
            down: true,
            pos: 0,
        };
        scr.rng[0] = 1;
        scr
    }
    fn default() -> Self {
        Self {
            rng: vec![0],
            down: false,
            pos: 0,
        }
    }
}
#[derive(Debug, Clone)]
struct Firewall {
    layers: Vec<Scanner>,
    packet: usize,
    pcs: Vec<usize>,
    svt: usize,
}

impl fmt::Display for Firewall {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for scan in &self.layers {
            for cell in scan.rng.iter() {
                write!(f, "[{}]", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl Firewall {
    fn new() -> Self {
        Self {
            layers: Vec::new(),
            packet: 0,
            pcs: Vec::new(),
            svt: 0,
        }
    }
    fn move_scanners(&mut self) {
        self.pcs.iter().for_each(|&y| {
            let r = &mut self.layers[y];
            if r.down {
                r.rng.swap(r.pos, r.pos + 1);
                r.pos += 1;
                if r.pos == r.rng.len() - 1 {r.down = false}
            } else {
                r.rng.swap(r.pos, r.pos - 1);
                r.pos -= 1;
                if r.pos == 0 {r.down = true}
            }
        })
    }
    fn move_packet(&mut self) {
        let p = self.packet;
        let l = &self.layers;
        if p < l.len() {
            if l[p].rng[0] == 1 {self.svt += p * l[p].rng.len()}
            self.packet += 1
        }
    }
    fn round(&mut self) {
        while self.packet < self.layers.len() {
            self.move_packet();
            self.move_scanners();
        }
    }
    fn find_delay(&self) -> usize {
        let layers: Vec<(usize, usize)> = self.pcs.iter().copied().zip(self.layers.iter().filter(|&r| r.rng.len() > 1).map(|r| r.rng.len())).collect();

        (0..)
        .find(|&delay| {
            !layers.iter().any(|(depth, rng)| {
                (depth + delay) % (2 * (rng - 1)) == 0
            })
        })
        .unwrap()
    }
}

fn parse_input() -> Firewall {
    let input: &str = include_str!("input13.txt");
    let mut frw = Firewall::new();
    let mut idx = 0;
    for line in input.lines() {
        let pts: Vec<usize> = line.split(": ").map(|x| x.parse::<usize>().unwrap()).collect();
        let (dep, rng) = (pts[0], pts[1]);
        while dep != idx {
            frw.layers.push(Scanner::default());
            idx += 1;
        }

        frw.layers.push(Scanner::new(rng));
        frw.pcs.push(dep);
        idx += 1
    }
    frw
}

fn part1() -> usize {
    let mut frw = parse_input();
    frw.round();
    frw.svt
}

fn part2() -> usize {
    let frw = parse_input();
    frw.find_delay()
}

fn main() {
    println!("{}", part2());
}