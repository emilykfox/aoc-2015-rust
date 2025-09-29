use std::{collections::HashMap, io::stdin};

#[derive(Clone)]
enum InputType {
    Direct,
    And,
    Or,
    LShift,
    RShift,
    Not,
}

#[derive(Clone)]
struct Wire<'a> {
    signal: Option<u16>,
    input_type: InputType,
    left: Option<&'a str>,
    right: &'a str,
}

fn main() {
    let re = regex::Regex::new(r"(?:(?<left>[a-z0-9]+) )?(?:(?<and>AND )|(?<or>OR )|(?<lshift>LSHIFT )|(?<rshift>RSHIFT )|(?<not>NOT )|^)(?<right>[a-z0-9]+) -> (?<to>[a-z]+)").unwrap();

    let lines = stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let mut wires = HashMap::<&str, Wire>::new();
    for line in lines.iter() {
        let captures = re.captures(line).expect(line);

        let input_type = if captures.name("and").is_some() {
            InputType::And
        } else if captures.name("or").is_some() {
            InputType::Or
        } else if captures.name("lshift").is_some() {
            InputType::LShift
        } else if captures.name("rshift").is_some() {
            InputType::RShift
        } else if captures.name("not").is_some() {
            InputType::Not
        } else {
            InputType::Direct
        };

        let wire = Wire {
            signal: None,
            input_type,
            left: captures.name("left").map(|found| found.as_str()),
            right: captures.name("right").unwrap().as_str(),
        };

        wires.insert(captures.name("to").unwrap().as_str(), wire);
    }

    println!("Solution: {}", get_signal("a", &mut wires));
    // println!("Part 2: {}", total_brightness);
}

fn get_signal(name: &str, wires: &mut HashMap<&str, Wire>) -> u16 {
    let wire = &wires[name];
    if let Some(signal) = wire.signal {
        return signal;
    }

    let mut wire = wire.clone();
    let left_signal = wire.left.as_ref().map(|left| {
        left.parse::<u16>()
            .unwrap_or_else(|_| get_signal(left, wires))
    });
    let right_signal = wire
        .right
        .parse::<u16>()
        .unwrap_or_else(|_| get_signal(wire.right, wires));

    let signal = match wire.input_type {
        InputType::Direct => right_signal,
        InputType::And => left_signal.unwrap() & right_signal,
        InputType::Or => left_signal.unwrap() | right_signal,
        InputType::LShift => left_signal.unwrap() << right_signal,
        InputType::RShift => left_signal.unwrap() >> right_signal,
        InputType::Not => !right_signal,
    };

    wire.signal = Some(signal);
    *wires.get_mut(name).unwrap() = wire;
    signal
}
