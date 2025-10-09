use std::{collections::HashMap, io::stdin};

fn main() {
    let instructions = stdin()
        .lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    let mut registers = HashMap::<char, u64>::new();
    registers.insert('a', 0);
    registers.insert('b', 0);
    let mut pc = 0;

    while pc < instructions.len() {
        let parts = instructions[pc]
            .split(&[' '])
            .filter(|part| !part.is_empty())
            .collect::<Vec<&str>>();

        match parts[0] {
            "hlf" => {
                *registers
                    .get_mut(&parts[1].chars().next().unwrap())
                    .unwrap() /= 2;
                pc += 1;
            }
            "tpl" => {
                *registers
                    .get_mut(&parts[1].chars().next().unwrap())
                    .unwrap() *= 3;
                pc += 1;
            }
            "inc" => {
                *registers
                    .get_mut(&parts[1].chars().next().unwrap())
                    .unwrap() += 1;
                pc += 1;
            }
            "jmp" => pc = (pc as isize + parts[1].parse::<isize>().unwrap()) as usize,
            "jie" => {
                if registers[&parts[1].chars().next().unwrap()] % 2 == 0 {
                    pc = (pc as isize + parts[2].parse::<isize>().unwrap()) as usize
                } else {
                    pc += 1
                }
            }
            "jio" => {
                if registers[&parts[1].chars().next().unwrap()] == 1 {
                    pc = (pc as isize + parts[2].parse::<isize>().unwrap()) as usize
                } else {
                    pc += 1
                }
            }
            _ => panic!("Bad instruction!"),
        };
    }

    let b = registers[&'b'];

    println!("Part 1: {b}");

    let mut registers = HashMap::<char, u64>::new();
    registers.insert('a', 1);
    registers.insert('b', 0);
    let mut pc = 0;

    while pc < instructions.len() {
        let parts = instructions[pc]
            .split(&[' '])
            .filter(|part| !part.is_empty())
            .collect::<Vec<&str>>();

        match parts[0] {
            "hlf" => {
                *registers
                    .get_mut(&parts[1].chars().next().unwrap())
                    .unwrap() /= 2;
                pc += 1;
            }
            "tpl" => {
                *registers
                    .get_mut(&parts[1].chars().next().unwrap())
                    .unwrap() *= 3;
                pc += 1;
            }
            "inc" => {
                *registers
                    .get_mut(&parts[1].chars().next().unwrap())
                    .unwrap() += 1;
                pc += 1;
            }
            "jmp" => pc = (pc as isize + parts[1].parse::<isize>().unwrap()) as usize,
            "jie" => {
                if registers[&parts[1].chars().next().unwrap()] % 2 == 0 {
                    pc = (pc as isize + parts[2].parse::<isize>().unwrap()) as usize
                } else {
                    pc += 1
                }
            }
            "jio" => {
                if registers[&parts[1].chars().next().unwrap()] == 1 {
                    pc = (pc as isize + parts[2].parse::<isize>().unwrap()) as usize
                } else {
                    pc += 1
                }
            }
            _ => panic!("Bad instruction!"),
        };
    }

    let b = registers[&'b'];

    println!("Part 2: {b}");
}
