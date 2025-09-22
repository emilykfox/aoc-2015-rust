use std::io;

fn main() {
    let input = io::stdin().lines().next().unwrap().unwrap();

    let mut current_floor = 0;
    let mut first_basement = None;
    let mut position = 0;
    input.chars().for_each(|paren| {
        position += 1;
        match paren {
            '(' => current_floor += 1,
            _ => current_floor -= 1,
        }

        if first_basement.is_none() && current_floor < 0 {
            first_basement = Some(position);
        }
    });

    println!("Part 1: {}", current_floor);
    println!("Part 2: {}", first_basement.unwrap());
}
