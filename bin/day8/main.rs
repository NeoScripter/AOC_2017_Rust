use std::collections::HashMap;
use std::ops::{Add, Sub};

fn solve(input: &str) -> (i32, i32) {
    let mut registers: HashMap<&str, i32> = HashMap::new();
    let mut largest_value: i32 = 0;
    for line in input.lines() {
        let (reg, _) = line.split_once(" ").unwrap();
        registers.insert(reg, 0);
    }
    for line in input.lines() {
        let ins: Vec<&str> = line.split_whitespace().collect();
        if ins.len() < 7 {
            panic!("Invalid input format");
        }
        let (register, op, value, _, con_key, con_sign, con_value) = 
            (ins[0], ins[1], ins[2].parse::<i32>().unwrap(), ins[3], ins[4], ins[5], ins[6].parse::<i32>().unwrap());
        let register_value = *registers.get(con_key).unwrap();
        let cond = match con_sign {
            "<" => i32::lt,
            ">" => i32::gt,
            ">=" => i32::ge,
            "<=" => i32::le,
            "==" => i32::eq,
            "!=" => i32::ne,
            _ => panic!("invalid cond: {}", con_sign),
        };

        let condition = cond(&register_value, &con_value);

        if condition {
            let operation = match op {
                "inc" => i32::add,
                "dec" => i32::sub,
                _ => panic!("invalid op: {}", op),
            };

            let entry = registers.entry(register).or_insert(0);
            *entry = operation(*entry, value);

            if *entry > largest_value {
                largest_value = *entry;
            }
        }
    }
    let max_in_registers = *registers.values().max().unwrap_or(&0);
    (max_in_registers, largest_value)
}
fn main() {
    let input = include_str!("input8.txt");
    let part1 = solve(input).0;
    let part2 = solve(input).1;
    println!("{}, {}", part1, part2);
}