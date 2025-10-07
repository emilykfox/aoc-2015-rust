use std::io::stdin;

use itertools::{Itertools, iproduct};

struct Item {
    cost: i16,
    damage: i16,
    armor: i16,
}

struct Fighter {
    hit_points: i16,
    damage: i16,
    armor: i16,
}

const SHOP: &str = "\
Weapons:    Cost  Damage  Armor
Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0

Armor:      Cost  Damage  Armor
Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5

Rings:      Cost  Damage  Armor
Damage +1    25     1       0
Damage +2    50     2       0
Damage +3   100     3       0
Defense +1   20     0       1
Defense +2   40     0       2
Defense +3   80     0       3";

fn main() {
    let item_re =
        regex::Regex::new(r#"^\w+(?: \+\d)?\s+(?<cost>\d+)\s+(?<damage>\d+)\s+(?<armor>\d+)$"#)
            .unwrap();
    let mut shop_lines = SHOP.split('\n');

    let weapons = shop_lines
        .by_ref()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let captures = item_re.captures(line).expect(line);
            Item {
                cost: captures["cost"].parse::<i16>().unwrap(),
                damage: captures["damage"].parse::<i16>().unwrap(),
                armor: captures["armor"].parse::<i16>().unwrap(),
            }
        })
        .collect::<Vec<Item>>();

    let armor = shop_lines
        .by_ref()
        .skip(1)
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let captures = item_re.captures(line).expect(line);
            Item {
                cost: captures["cost"].parse::<i16>().unwrap(),
                damage: captures["damage"].parse::<i16>().unwrap(),
                armor: captures["armor"].parse::<i16>().unwrap(),
            }
        })
        .chain([Item {
            cost: 0,
            damage: 0,
            armor: 0,
        }])
        .collect::<Vec<Item>>();

    let rings = shop_lines
        .skip(1)
        .map(|line| {
            let captures = item_re.captures(line).expect(line);
            Item {
                cost: captures["cost"].parse::<i16>().unwrap(),
                damage: captures["damage"].parse::<i16>().unwrap(),
                armor: captures["armor"].parse::<i16>().unwrap(),
            }
        })
        .chain([
            Item {
                cost: 0,
                damage: 0,
                armor: 0,
            },
            Item {
                cost: 0,
                damage: 0,
                armor: 0,
            },
        ])
        .collect::<Vec<Item>>();

    let boss_re = regex::Regex::new(r#"\d+"#).unwrap();
    let mut lines = stdin().lines().map_while(Result::ok);
    let boss_hit_points = boss_re
        .find(&lines.next().unwrap())
        .unwrap()
        .as_str()
        .parse::<i16>()
        .unwrap();
    let boss_damage = boss_re
        .find(&lines.next().unwrap())
        .unwrap()
        .as_str()
        .parse::<i16>()
        .unwrap();
    let boss_armor = boss_re
        .find(&lines.next().unwrap())
        .unwrap()
        .as_str()
        .parse::<i16>()
        .unwrap();
    let boss = Fighter {
        hit_points: boss_hit_points,
        damage: boss_damage,
        armor: boss_armor,
    };

    let product = iproduct!(weapons.iter(), armor.iter(), rings.iter().combinations(2));
    let (min, max) = product
        .map(|items| {
            let player = Fighter {
                hit_points: 100,
                damage: items.0.damage + items.1.damage + items.2[0].damage + items.2[1].damage,
                armor: items.0.armor + items.1.armor + items.2[0].armor + items.2[1].armor,
            };
            if player_wins(&player, &boss) {
                (
                    Some(items.0.cost + items.1.cost + items.2[0].cost + items.2[1].cost),
                    None,
                )
            } else {
                (
                    None,
                    Some(items.0.cost + items.1.cost + items.2[0].cost + items.2[1].cost),
                )
            }
        })
        .reduce(|(acc_min, acc_max), (win_cost, lose_cost)| {
            (
                acc_min
                    .and_then(|acc_min| {
                        win_cost
                            .map(|win_cost| acc_min.min(win_cost))
                            .or(Some(acc_min))
                    })
                    .or(win_cost),
                acc_max.max(lose_cost),
            )
        })
        .map(|(min, max)| (min.unwrap(), max.unwrap()))
        .unwrap();

    println!("Part 1: {min}");

    println!("Part 2: {max}");
}

fn player_wins(player: &Fighter, boss: &Fighter) -> bool {
    let mut player_hit_points = player.hit_points;
    let mut boss_hit_points = boss.hit_points;
    loop {
        boss_hit_points -= (player.damage - boss.armor).max(1);
        if boss_hit_points <= 0 {
            return true;
        }
        player_hit_points -= (boss.damage - player.armor).max(1);
        if player_hit_points <= 0 {
            return false;
        }
    }
}
