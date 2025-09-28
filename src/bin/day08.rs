use std::io::stdin;

fn main() {
    let mut total_code = 0;
    let mut total_values = 0;
    let mut total_reencode = 0;
    for line in stdin().lines().map_while(Result::ok) {
        let mut chars = line.chars();

        total_code += 1; // "
        _ = chars.next();
        while let Some(char) = chars.next() {
            total_code += 1;
            if char != '"' {
                total_values += 1;
            }

            if char == '\\' {
                total_code += 1;
                if chars.next().unwrap() == 'x' {
                    total_code += 2;
                    _ = chars.next();
                    _ = chars.next()
                }
            }
        }

        total_reencode += 2
            + line.len()
            + line
                .chars()
                .filter(|&char| char == '\\' || char == '"')
                .count();
    }

    let total_waste = total_code - total_values;
    let reencode_waste = total_reencode - total_code;
    println!("Part 1: {}", total_waste);
    println!("Part 2: {}", reencode_waste);
}
