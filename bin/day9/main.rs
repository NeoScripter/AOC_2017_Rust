fn part1(input: &str) -> u32 {
    let mut acc = 0;
    let mut is_garbage = false;
    let mut parenth_level = 0;
    let mut skip_next = false; 

    for c in input.chars() {
        if skip_next {
            skip_next = false;  
            continue;
        }

        match c {
            '!' if is_garbage => skip_next = true,
            '<' => is_garbage = true,
            '>' => is_garbage = false,
            '{' if !is_garbage => {
                parenth_level += 1;
                acc += parenth_level;
            },
            '}' if !is_garbage => parenth_level -= 1,
            _ => {},
        }
    }

    acc
}
fn part2(input: &str) -> u32 {
    let mut is_garbage = false;
    let mut garbage_char = 0;
    let mut skip_next = false; 

    for c in input.chars() {
        if skip_next {
            skip_next = false;  
            continue;
        }

        match c {
            '!' if is_garbage => skip_next = true,
            '<' if is_garbage => garbage_char += 1,
            '>' if !is_garbage => garbage_char += 1,
            '<' => is_garbage = true,
            '>' => is_garbage = false,
            _ if is_garbage => garbage_char += 1,
            _ => {},
        }
    }

    garbage_char
}
fn main() {
    let input = include_str!("input9.txt");
    println!("{}, {}", part1(input), part2(input));
}