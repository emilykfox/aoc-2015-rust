use std::io::stdin;

fn main() {
    let sum_re = regex::Regex::new(r"-?\d+").unwrap();
    let not_red_sum_re = regex::Regex::new(r#"(?<start_array>\[)|(?<end_array>\])|(?<start_object>\{)|(?<end_object>\})|(?<red>"red")|(?<number>-?\d+)"#).unwrap();

    let input = stdin().lines().next().unwrap().unwrap();

    let sum_matches = sum_re.find_iter(&input);
    let sum = sum_matches
        .map(|number_match| number_match.as_str().parse::<i32>().unwrap())
        .sum::<i32>();

    println!("Part 1: {sum}");

    let mut tokens = not_red_sum_re.captures_iter(&input);
    let not_red_sum = get_not_red_sum(&mut tokens, false);

    println!("Part 2: {not_red_sum}");
}

fn get_not_red_sum(tokens: &mut regex::CaptureMatches, in_object: bool) -> i32 {
    let mut sum = 0;
    let mut red = false;
    while let Some(token) = tokens.next() {
        if token.name("start_array").is_some() {
            sum += get_not_red_sum(tokens, false);
        } else if token.name("end_array").is_some() {
            assert!(!in_object);
            break;
        } else if token.name("start_object").is_some() {
            sum += get_not_red_sum(tokens, true);
        } else if token.name("end_object").is_some() {
            assert!(in_object);
            break;
        } else if token.name("red").is_some() {
            if in_object {
                red = true;
            }
        } else {
            sum += token
                .name("number")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .unwrap();
        }
    }

    if red { 0 } else { sum }
}
