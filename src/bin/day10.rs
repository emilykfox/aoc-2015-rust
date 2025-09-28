use std::io::stdin;

fn main() {
    let start = stdin().lines().next().unwrap().unwrap();

    let mut current = start;
    let mut at_forty: Option<usize> = None;
    for i in 0..50 {
        let mut next = String::new();
        let mut reading: Option<char> = None;
        let mut num_read = 0usize;
        for character in current.chars() {
            if Some(character) == reading {
                num_read += 1;
            } else {
                if let Some(reading) = reading {
                    next.push_str(&num_read.to_string());
                    next.push(reading);
                }
                reading = Some(character);
                num_read = 1;
            }
        }
        next.push_str(&num_read.to_string());
        next.push(reading.unwrap());

        current = next;
        if i == 39 {
            at_forty = Some(current.len());
        }
    }

    println!("Part 1: {}", at_forty.unwrap());

    println!("Part 2: {}", current.len());
}
