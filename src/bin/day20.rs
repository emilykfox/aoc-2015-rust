use std::io::stdin;

fn main() {
    let target = stdin()
        .lines()
        .next()
        .unwrap()
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let start = f64::sqrt(target as f64 / 10.0) as u64;

    let first = (start..)
        .find(|house| {
            (1..=(f64::sqrt(*house as f64) as u64))
                .map(|elf| {
                    if house % elf == 0 {
                        10 * elf + 10 * (house / elf)
                    } else {
                        0
                    }
                })
                .sum::<u64>()
                >= target
        })
        .unwrap();
    println!("Part 1: {first}");

    let start = f64::sqrt(target as f64 / 11.0) as u64;

    let first = (start..)
        .find(|house| {
            (1..=(f64::sqrt(*house as f64) as u64))
                .map(|elf| {
                    if house % elf == 0 {
                        if house / elf <= 50 {
                            11 * elf
                        } else if elf <= 50 {
                            11 * (house / elf)
                        } else {
                            0
                        }
                    } else {
                        0
                    }
                })
                .sum::<u64>()
                >= target
        })
        .unwrap();

    println!("Part 2: {first}");
}
