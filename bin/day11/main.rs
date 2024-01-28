use std::cmp;

fn part1(input: &str) -> i32 {
    let (mut x, mut y, mut z) = (0i32, 0i32, 0i32);

    let mut max_dist = 0;
    input.split(",").for_each(|dir| {
        match dir {
            "nw" => { x -= 1; y += 1; },
            "n" => { y += 1; z -= 1; },
            "ne"  => { x += 1; z -= 1; },
            "se" => { x += 1; y -= 1; },
            "s" => { y -= 1; z += 1; },
            "sw"  => { x -= 1; z += 1; },
            _    => (),
        }
        let cur_dist = cmp::max(x.abs(), cmp::max(y.abs(), z.abs()));
        max_dist = max_dist.max(cur_dist);
    });
    max_dist
}

fn main() {
    let input = include_str!("input11.txt");
    println!("{}", part1(input));
}