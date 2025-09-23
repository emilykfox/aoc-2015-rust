use std::io::stdin;

struct Measurements {
    length: u32,
    width: u32,
    height: u32,
}

fn main() {
    let measurements = stdin()
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            let mut parts = line.split('x');
            let length = parts.next()?.parse::<u32>().ok()?;
            let width = parts.next()?.parse::<u32>().ok()?;
            let height = parts.next()?.parse::<u32>().ok()?;
            Some(Measurements {
                length,
                width,
                height,
            })
        })
        .collect::<Vec<Measurements>>();

    let paper_needed = measurements
        .iter()
        .map(|measurements| {
            let length_width = measurements.length * measurements.width;
            let width_height = measurements.width * measurements.height;
            let length_height = measurements.length * measurements.height;

            2 * (length_width + width_height + length_height)
                + length_width.min(width_height).min(length_height)
        })
        .sum::<u32>();

    println!("Part 1: {}", paper_needed);

    let ribbon_needed = measurements
        .iter()
        .map(|measurements| {
            2 * (measurements.length + measurements.width + measurements.height
                - measurements
                    .length
                    .max(measurements.width)
                    .max(measurements.height))
                + measurements.length * measurements.width * measurements.height
        })
        .sum::<u32>();

    println!("Part 2: {}", ribbon_needed);
}
