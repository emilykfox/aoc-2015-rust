use std::io::stdin;

use itertools::Itertools;

fn main() {
    let mut original_grid = [['.'; 102]; 102];
    for (row_index_zero, line) in stdin().lines().map_while(Result::ok).enumerate() {
        original_grid[row_index_zero + 1][1..=100]
            .copy_from_slice(&line.chars().collect::<Vec<char>>());
    }
    let original_grid = original_grid;

    let mut grid = original_grid;
    for _ in 1..=100 {
        let mut next_grid = grid;
        for row_index in 1..=100 {
            for column_index in 1..=100 {
                let deltas = (0..=2).cartesian_product(0..=2);
                let num_lit = deltas
                    .filter(|delta| *delta != (1, 1))
                    .filter(|(row_delta, column_delta)| {
                        grid[row_index + row_delta - 1][column_index + column_delta - 1] == '#'
                    })
                    .count();
                if grid[row_index][column_index] == '#' {
                    if num_lit != 2 && num_lit != 3 {
                        next_grid[row_index][column_index] = '.';
                    }
                } else if num_lit == 3 {
                    next_grid[row_index][column_index] = '#';
                }
            }
        }
        grid = next_grid;
    }

    let num_lit = (1..=100)
        .cartesian_product(1..=100)
        .filter(|(row_index, column_index)| grid[*row_index][*column_index] == '#')
        .count();
    println!("Part 1: {num_lit}");

    grid = original_grid;
    grid[1][1] = '#';
    grid[1][100] = '#';
    grid[100][1] = '#';
    grid[100][100] = '#';
    for _ in 1..=100 {
        let mut next_grid = grid;
        for row_index in 1..=100 {
            for column_index in 1..=100 {
                if (row_index != 1 && row_index != 100)
                    || (column_index != 1 && column_index != 100)
                {
                    let deltas = (0..=2).cartesian_product(0..=2);
                    let num_lit = deltas
                        .filter(|delta| *delta != (1, 1))
                        .filter(|(row_delta, column_delta)| {
                            grid[row_index + row_delta - 1][column_index + column_delta - 1] == '#'
                        })
                        .count();
                    if grid[row_index][column_index] == '#' {
                        if num_lit != 2 && num_lit != 3 {
                            next_grid[row_index][column_index] = '.';
                        }
                    } else if num_lit == 3 {
                        next_grid[row_index][column_index] = '#';
                    }
                }
            }
        }
        grid = next_grid;
    }

    let num_lit = (1..=100)
        .cartesian_product(1..=100)
        .filter(|(row_index, column_index)| grid[*row_index][*column_index] == '#')
        .count();
    println!("Part 2: {num_lit}");
}
