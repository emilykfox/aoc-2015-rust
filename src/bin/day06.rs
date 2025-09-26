use std::io::stdin;

fn main() {
    let re =
        regex::Regex::new(r"(turn on|turn off|toggle) (\d+),(\d+) through (\d+),(\d+)").unwrap();

    let mut lit = [[false; 1000]; 1000];
    let mut brightness = vec![vec![0; 1000]; 1000];

    for line in stdin().lines().map_while(Result::ok) {
        let captures = re.captures(&line).unwrap();
        let instruction = &captures[1];
        let low_row = captures[2].parse::<usize>().unwrap();
        let low_column = captures[3].parse::<usize>().unwrap();
        let high_row = captures[4].parse::<usize>().unwrap();
        let high_column = captures[5].parse::<usize>().unwrap();

        for row in low_row..=high_row {
            for column in low_column..=high_column {
                match instruction {
                    "turn on" => {
                        lit[row][column] = true;
                        brightness[row][column] += 1;
                    }
                    "turn off" => {
                        lit[row][column] = false;
                        brightness[row][column] = brightness[row][column].max(1) - 1;
                    }
                    "toggle" => {
                        lit[row][column] = !lit[row][column];
                        brightness[row][column] += 2;
                    }
                    _ => panic!("Bad instruction: {}", instruction),
                }
            }
        }
    }

    let num_lit: usize = lit
        .iter()
        .map(|column| {
            column
                .iter()
                .map(|light| match light {
                    true => 1,
                    false => 0,
                })
                .sum::<usize>()
        })
        .sum();

    let total_brightness: usize = brightness
        .iter()
        .map(|column| column.iter().sum::<usize>())
        .sum();

    println!("Part 1: {}", num_lit);
    println!("Part 2: {}", total_brightness);
}
