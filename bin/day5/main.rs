fn part1(input: &str) -> usize {
    let mut maze: Vec<isize> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let mut curr_pos: isize = 0;
    let mut prev_pos: isize = 0;
    let mut steps = 0;
    let length = maze.len();
    loop {
        curr_pos += maze[curr_pos as usize];
        maze[prev_pos as usize] += 1;
        prev_pos = curr_pos;
        steps += 1;
        if curr_pos as usize >= length {
            break;
        }
    }
    steps
}
fn part2(input: &str) -> usize {
    let mut maze: Vec<isize> = input.split_whitespace().filter_map(|x| x.parse().ok()).collect();
    let mut curr_pos: isize = 0;
    let mut steps = 0;
    let length = maze.len() as isize;

    while curr_pos >= 0 && curr_pos < length {
        let jump_offset = match maze.get(curr_pos as usize) {
            Some(&offset) => offset,
            None => break,
        };
        
        if jump_offset >= 3 {
            maze[curr_pos as usize] -= 1;
        } else {
            maze[curr_pos as usize] += 1;
        }

        curr_pos += jump_offset;
        steps += 1;
    }

    steps
}

fn main() {
    let input = include_str!("input5.txt");
    println!("{}, {}", part1(input), part2(input));
}