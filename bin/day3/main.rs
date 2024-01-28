use std::collections::HashMap;

fn do_fill(grid: &mut HashMap<(i64, i64), u64>,
           pos: &mut (i64, i64),
           input: u64,
           len: u64,
           dir: (i64, i64))
           -> Option<u64> {
    for _ in 0..len {
        *pos = (pos.0 + dir.0, pos.1 + dir.1);
        let mut sum = 0;
        for p in vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)] {
            sum += *grid.entry((pos.0 + p.0, pos.1 + p.1)).or_insert(0);
        }
        grid.insert(*pos, sum);
        if sum > input {
            return Some(sum);
        }
    }
    return None;
}

fn find_iterations(target: u64) -> u64 {
    let mut current = 1;
    let mut next_increment = 8;
    let mut iterations = 0;

    while current < target {
        iterations += 1;
        current += next_increment;
        next_increment += 8;
    }

    iterations
}
fn find_lowest_number(power: u64) -> u64 {
    let mut counter = 0;
    let mut current = 1;
    let mut next_increment = 8;
    loop {
        current += next_increment;
        next_increment += 8;
        counter += 1;
        if counter == power - 1 {
            break;
        }
    }
    current
}
fn part1(target: u64) -> i64 {
    let power = find_iterations(target);
    let side_length = 2 * power;
    let remainder = target - find_lowest_number(power);
    let rem2 = remainder % side_length;
    let half_side_length = side_length / 2;
    (half_side_length as i64 - rem2 as i64).abs() + power as i64
}

// The part 2 solution is not mine
fn part2(input: u64) -> u64 {
    let mut grid = HashMap::new();
    let mut pos: (i64, i64) = (0, 0);
    grid.insert(pos, 1);
    let mut len = 1;

    loop {
        let solution = do_fill(&mut grid, &mut pos, input, len, (1, 0));
        if solution.is_some() {
            return solution.unwrap();
        }
        let solution = do_fill(&mut grid, &mut pos, input, len, (0, 1));
        if solution.is_some() {
            return solution.unwrap();
        }
        len += 1;
        let solution = do_fill(&mut grid, &mut pos, input, len, (-1, 0));
        if solution.is_some() {
            return solution.unwrap();
        }
        let solution = do_fill(&mut grid, &mut pos, input, len, (0, -1));
        if solution.is_some() {
            return solution.unwrap();
        }
        len += 1;
    }
}

fn main() {
    let input = 361527;
    println!("{}, {}", part1(input), part2(input));
}