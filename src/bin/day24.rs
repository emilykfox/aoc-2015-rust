use std::io::stdin;

fn main() {
    let weights = stdin()
        .lines()
        .map_while(Result::ok)
        .map(|line| line.parse::<usize>().unwrap())
        .collect::<Vec<usize>>();

    let target = weights.iter().sum::<usize>() / 3;

    let mut current_layer = vec![vec![Option::<(usize, usize)>::None; target + 1]; target + 1];
    current_layer[0][0] = Some((0, 1));
    for first_weight in (0..weights.len()).rev() {
        current_layer = (0..=target)
            .map(|capacity1| {
                (0..=target)
                    .map(|capacity2| {
                        [
                            current_layer[capacity1][capacity2],
                            if weights[first_weight] <= capacity2 {
                                current_layer[capacity1][capacity2 - weights[first_weight]]
                            } else {
                                None
                            },
                            if weights[first_weight] <= capacity1 {
                                current_layer[capacity1 - weights[first_weight]][capacity2]
                                    .map(|prev| (prev.0 + 1, prev.1 * weights[first_weight]))
                            } else {
                                None
                            },
                        ]
                        .into_iter()
                        .flatten()
                        .reduce(|acc, choice| {
                            if acc.0 < choice.0 || (acc.0 == choice.0 && acc.1 < choice.1) {
                                acc
                            } else {
                                choice
                            }
                        })
                    })
                    .collect::<Vec<Option<(usize, usize)>>>()
            })
            .collect::<Vec<Vec<Option<(usize, usize)>>>>();
    }

    let entanglement = current_layer[target][target].unwrap().1;
    println!("Part 1: {entanglement}");

    let target = weights.iter().sum::<usize>() / 4;

    let mut current_layer =
        vec![vec![vec![Option::<(usize, usize)>::None; target + 1]; target + 1]; target + 1];
    current_layer[0][0][0] = Some((0, 1));
    for first_weight in (0..weights.len()).rev() {
        current_layer = (0..=target)
            .map(|capacity1| {
                (0..=target)
                    .map(|capacity2| {
                        (0..=target)
                            .map(|capacity3| {
                                [
                                    current_layer[capacity1][capacity2][capacity3],
                                    if weights[first_weight] <= capacity3 {
                                        current_layer[capacity1][capacity2]
                                            [capacity3 - weights[first_weight]]
                                    } else {
                                        None
                                    },
                                    if weights[first_weight] <= capacity2 {
                                        current_layer[capacity1][capacity2 - weights[first_weight]]
                                            [capacity3]
                                    } else {
                                        None
                                    },
                                    if weights[first_weight] <= capacity1 {
                                        current_layer[capacity1 - weights[first_weight]][capacity2]
                                            [capacity3]
                                            .map(|prev| {
                                                (prev.0 + 1, prev.1 * weights[first_weight])
                                            })
                                    } else {
                                        None
                                    },
                                ]
                                .into_iter()
                                .flatten()
                                .reduce(|acc, choice| {
                                    if acc.0 < choice.0 || (acc.0 == choice.0 && acc.1 < choice.1) {
                                        acc
                                    } else {
                                        choice
                                    }
                                })
                            })
                            .collect::<Vec<Option<(usize, usize)>>>()
                    })
                    .collect::<Vec<Vec<Option<(usize, usize)>>>>()
            })
            .collect::<Vec<Vec<Vec<Option<(usize, usize)>>>>>();
    }

    let entanglement = current_layer[target][target][target].unwrap().1;
    println!("Part 2: {entanglement}");
}
