const LENGTH: usize = 256;
const ADD_LEN: [u8; 5] = [17, 31, 73, 47, 23];

fn part1(input: &str) -> u32 {
    let mut list = Vec::new();
    for n in 0..256 {
        list.push(n)
    }
    let mut shifted = 0;
    let mut skip_size = 0;
    input.split(',').for_each(|length| {
        let len = length.parse::<usize>().unwrap();
        list[0..0 + len].reverse();
        let step_shift = (len + skip_size) % LENGTH;
        list.rotate_left(step_shift);
        shifted += step_shift;
        skip_size += 1;
    });
    list.rotate_right(shifted % LENGTH);
    list[0] * list[1]
}

fn part2(input: &str) -> String {
    let mut list = Vec::new();
    for n in 0..=255 {
        list.push(n)
    }
    let mut sequence: Vec<u8> = input.bytes().collect();
    sequence.extend(ADD_LEN);
    let mut shifted = 0;
    let mut skip_size = 0;

    for _ in 0..64 {
        for &length in &sequence {
            let len = length as usize;
            list[0..len % LENGTH].reverse();
            let step_shift = (len + skip_size) % LENGTH;
            list.rotate_left(step_shift);
            shifted += step_shift;
            skip_size += 1;
        }
    }
    list.rotate_right(shifted % LENGTH);
    list
    .chunks(16)
    .map(|chunk| chunk.iter().fold(0, |acc, &x| acc ^ x))
    .map(|n| format!("{:02x}", n))
    .collect::<String>()
}
fn main() {
    let input = include_str!("input10.txt");
    println!("{}", part2(input));
}