use std::{collections::HashSet, io::stdin};

fn main() {
    let directions = stdin().lines().next().unwrap().unwrap();

    let mut visited = HashSet::<(i32, i32)>::new();
    let mut current = (0, 0);
    visited.insert(current);

    for direction in directions.chars() {
        let (row_delta, column_delta) = match direction {
            '^' => (1, 0),
            'v' => (-1, 0),
            '>' => (0, 1),
            _ => (0, -1),
        };
        current = (current.0 + row_delta, current.1 + column_delta);
        visited.insert(current);
    }

    println!("Part 1: {}", visited.len());

    visited = HashSet::new();
    let mut santa_current = (0, 0);
    let mut robot_current = (0, 0);

    visited.insert(santa_current);
    let mut santa_next = true;
    for direction in directions.chars() {
        let (row_delta, column_delta) = match direction {
            '^' => (1, 0),
            'v' => (-1, 0),
            '>' => (0, 1),
            _ => (0, -1),
        };

        if santa_next {
            santa_current = (santa_current.0 + row_delta, santa_current.1 + column_delta);
            visited.insert(santa_current);
        } else {
            robot_current = (robot_current.0 + row_delta, robot_current.1 + column_delta);
            visited.insert(robot_current);
        }
        santa_next = !santa_next;
    }

    println!("Part 2: {}", visited.len());
}
