use std::io::stdin;

fn main() {
    let re = regex::Regex::new(r#"\d+"#).unwrap();
    let line = stdin().lines().next().unwrap().unwrap();
    let mut matches = re.find_iter(&line);

    let target_row = matches.next().unwrap().as_str().parse::<usize>().unwrap();
    let target_column = matches.next().unwrap().as_str().parse::<usize>().unwrap();

    let mut code: u128 = 20151125;
    'diagonals: for diagonal in 1.. {
        for row in (1..=diagonal).rev() {
            let column = diagonal + 1 - row;
            if row == target_row && column == target_column {
                break 'diagonals;
            }

            code = (code * 252533) % 33554393;
        }
    }

    println!("Part 1: {code}");

    println!("Part 2: You win!!!");
}
