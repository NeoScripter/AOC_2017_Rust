use std::collections::{VecDeque, HashSet}; 
use itertools::Itertools;

const LENGTH: usize = 256;
const ADD_LEN: [u8; 5] = [17, 31, 73, 47, 23];

#[derive(Debug, Clone)]
struct Disk<'a> {
    salt: &'a str,
    grid: Vec<Vec<u32>>,
}

impl<'a> Disk<'a> {
    fn new(salt: &'a str) -> Self {
        Self {
            salt,
            grid: Vec::new(),
        }
    }
    fn find_regions(&self) -> u32 {
        let wth = self.grid[0].len();
        let hgt = self.grid.len();
        let mut sum = 0;
        let mut cache = HashSet::new();
        (0..hgt).for_each(|y| {
            (0..wth).for_each(|x| {
                let point = self.grid[y][x];
                if point > 0 && !cache.contains(&(y, x)) {
                    let mut q = VecDeque::new();
                    q.push_back((y, x));
                    while let Some((y2, x2)) = q.pop_front() {
                        if !cache.insert((y2, x2)) { continue }

                        if x2 > 0 { if self.grid[y2][x2 - 1] > 0 { q.push_back((y2, x2 - 1)) } }
                        if x2 < wth - 1 { if self.grid[y2][x2 + 1] > 0 { q.push_back((y2, x2 + 1)) } }
                        if y2 > 0 { if self.grid[y2 - 1][x2] > 0 { q.push_back((y2 - 1, x2)) } }
                        if y2 < hgt - 1 { if self.grid[y2 + 1][x2] > 0 { q.push_back((y2 + 1, x2)) } }
                    }
                    sum += 1;
                }
            })
        });
        sum
    }
    fn find_rows(&mut self) {
        for n in 0..128 {
            let output = self.knot_hash(format!("{}-{}", self.salt, n));
            let mut row = Vec::new();

            for c in output.chars() {
                if let Some(d) = c.to_digit(16) {
                    for i in (0..4).rev() {
                        row.push(((d >> i) & 1) as u32);
                    }
                }
            }
            //println!("{}, {}", n, row);
            self.grid.push(row);
        }
        self.grid.reverse();
    }
    fn find_squares(&self) -> u32 {
        self.grid.iter().flatten().sum()
    }
    fn knot_hash(&self, input: String) -> String {
        let mut list = Vec::new();
        for n in 0..=255 {
            list.push(n)
        }
        let mut seq: Vec<u8> = input.bytes().collect();
        seq.extend(ADD_LEN);
        let mut shifted = 0;
        let mut skip = 0;
    
        for _ in 0..64 {
            for &length in &seq {
                let len = length as usize;
                list[0..len % LENGTH].reverse();
                let step = (len + skip) % LENGTH;
                list.rotate_left(step);
                shifted += step;
                skip += 1;
            }
        }
        list.rotate_right(shifted % LENGTH);
        list
        .chunks(16)
        .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
        .map(|n| format!("{:02x}", n))
        .collect::<String>()
    }
}


fn part1(input: &str) -> u32 {
    let mut disk = Disk::new(input);
    disk.find_rows();
    disk.find_squares()
}

fn part2(input: &str) -> u32 {
    let mut disk = Disk::new(input);
    disk.find_rows();
    disk.find_regions()
}

fn main() {
    println!("{}", part1("vbqugkhl"));
}