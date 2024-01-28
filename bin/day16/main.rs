use std::collections::HashMap;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Clone)]
struct Dance(Vec<char>);

enum Command {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for Command {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next() {
            Some('s') => s[1..].parse().map(Command::Spin),
            Some('x') => {
                let parts: Vec<_> = s[1..].split('/').collect();
                Ok(Command::Exchange(parts[0].parse()?, parts[1].parse()?))
            }
            Some('p') => {
                let mut chars = s[1..].chars();
                Ok(Command::Partner(chars.next().unwrap(), chars.skip(1).next().unwrap()))
            }
            _ => unreachable!(),
        }
    }
}

impl Dance {
    fn new(row: Vec<char>) -> Self {
        Self(row)
    }

    fn apply_command(&mut self, cmd: &Command) {
        match cmd {
            Command::Spin(x) => self.0.rotate_right(*x),
            Command::Exchange(a, b) => self.0.swap(*a, *b),
            Command::Partner(a, b) => {
                let pos_a = self.0.iter().position(|&x| x == *a).unwrap();
                let pos_b = self.0.iter().position(|&x| x == *b).unwrap();
                self.0.swap(pos_a, pos_b);
            },
        }
    }

    fn result(&self) -> String {
        self.0.iter().collect()
    }
}

fn part1(commands: &[Command]) -> String {
    let mut dance = Dance::new(('a'..='p').collect());
    for cmd in commands {
        dance.apply_command(cmd);
    }
    dance.result()
}

fn part2(commands: &[Command], iterations: usize) -> String {
    let mut dance = Dance::new(('a'..='p').collect());
    let mut cache = HashMap::new();

    for n in 1..=iterations {
        for cmd in commands {
            dance.apply_command(cmd);
        }

        let result = dance.result();
        if let Some(&old) = cache.get(&result) {
            if (iterations - n) % (n - old) == 0 {
                return result;
            }
        }
        cache.insert(result, n);
    }

    unreachable!()
}

fn main() {
    let commands: Vec<Command> = include_str!("input16.txt")
        .split(',')
        .filter_map(|cmd| cmd.parse().ok())
        .collect();

    println!("Part 1: {}", part1(&commands));
    println!("Part 2: {}", part2(&commands, 1_000_000_000));
}
