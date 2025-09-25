use std::io::stdin;

fn main() {
    let input = stdin().lines().next().unwrap().unwrap();

    let mut part_1: Option<u32> = None;
    let mut part_2: Option<u32> = None;
    for i in 1.. {
        let to_hash = format!("{}{}", input, i);
        let digest = format!("{:x}", md5::compute(to_hash.as_bytes()));
        if part_1.is_none() && digest.starts_with("00000") {
            part_1 = Some(i);
        }
        if digest.starts_with("000000") {
            part_2 = Some(i);
            break;
        }
    }

    println!("Part 1: {}", part_1.unwrap());
    println!("Part 2: {}", part_2.unwrap());
}
