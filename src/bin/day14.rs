use std::{collections::HashMap, io::stdin};

struct Reindeer {
    speed: u16,
    fly_duration: u16,
    rest_duration: u16,

    fly_time: u16,
    rest_time: u16,
    total_distance: u16,
    total_points: u16,
}

impl Reindeer {
    fn new(speed: u16, fly_duration: u16, rest_duration: u16) -> Reindeer {
        Reindeer {
            speed,
            fly_duration,
            rest_duration,
            fly_time: 0,
            rest_time: rest_duration,
            total_distance: 0,
            total_points: 0,
        }
    }
}

fn main() {
    let re = regex::Regex::new(r#"^\w+ can fly (?<speed>\d+) km/s for (?<fly_duration>\d+) seconds, but then must rest for (?<rest_duration>\d+) seconds.$"#).unwrap();

    let mut reindeer = Vec::<Reindeer>::new();
    for line in stdin().lines().map_while(Result::ok) {
        let captures = re.captures(&line).unwrap();
        reindeer.push(Reindeer::new(
            captures["speed"].parse::<u16>().unwrap(),
            captures["fly_duration"].parse::<u16>().unwrap(),
            captures["rest_duration"].parse::<u16>().unwrap(),
        ));
    }

    for _ in 0..2503 {
        for deer in reindeer.iter_mut() {
            if deer.rest_time == deer.rest_duration {
                deer.rest_time = 0;
                deer.fly_time = 0;
            }

            if deer.fly_time == deer.fly_duration {
                deer.rest_time += 1;
            } else {
                deer.total_distance += deer.speed;
                deer.fly_time += 1;
            }
        }

        let max_distance = reindeer
            .iter()
            .map(|deer| deer.total_distance)
            .max()
            .unwrap();
        for deer in reindeer.iter_mut() {
            if deer.total_distance == max_distance {
                deer.total_points += 1;
            }
        }
    }

    let max_distance = reindeer
        .iter()
        .map(|deer| deer.total_distance)
        .max()
        .unwrap();

    println!("Part 1: {max_distance}");

    let max_points = reindeer.iter().map(|deer| deer.total_points).max().unwrap();

    println!("Part 2: {max_points}");
}
