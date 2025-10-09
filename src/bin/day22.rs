use std::{collections::HashMap, io::stdin};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct MinManaParams {
    player_hit_points: i16,
    player_mana: i16,

    boss_hit_points: i16,
    boss_damage: i16,

    shield_timer: u8,
    poison_timer: u8,
    recharge_timer: u8,

    player_turn: bool,

    hard_mode: bool,
}

impl MinManaParams {
    fn new(boss_hit_points: i16, boss_damage: i16, hard_mode: bool) -> Self {
        MinManaParams {
            player_hit_points: 50,
            player_mana: 500,
            boss_hit_points,
            boss_damage,
            shield_timer: 0,
            poison_timer: 0,
            recharge_timer: 0,
            player_turn: true,
            hard_mode,
        }
    }
}

fn main() {
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

    let params = MinManaParams::new(boss_hit_points, boss_damage, false);
    let min = min_mana(params, &mut HashMap::new()).unwrap();
    println!("Part 1: {min}");

    let params = MinManaParams::new(boss_hit_points, boss_damage, true);
    let min = min_mana(params, &mut HashMap::new()).unwrap();
    println!("Part 2: {min}");
}

fn min_mana(params: MinManaParams, table: &mut HashMap<MinManaParams, Option<u16>>) -> Option<u16> {
    table
        .get(&params)
        .copied()
        .or_else(|| {
            let mut new_params = params;
            let armor = if new_params.shield_timer > 0 {
                new_params.shield_timer -= 1;
                7
            } else {
                0
            };
            if new_params.poison_timer > 0 {
                new_params.poison_timer -= 1;
                new_params.boss_hit_points -= 3;
            }
            if new_params.recharge_timer > 0 {
                new_params.recharge_timer -= 1;
                new_params.player_mana += 101;
            }

            let result = if new_params.boss_hit_points <= 0 {
                Some(0)
            } else if new_params.player_turn {
                if new_params.hard_mode {
                    new_params.player_hit_points -= 1;
                }

                if new_params.player_hit_points <= 0 {
                    None
                } else {
                    new_params.player_turn = false;
                    [
                        // Magic missle
                        if new_params.player_mana >= 53 {
                            let mut choice_params = new_params;
                            choice_params.player_mana -= 53;
                            choice_params.boss_hit_points -= 4;
                            if choice_params.boss_hit_points <= 0 {
                                Some(53)
                            } else {
                                min_mana(choice_params, table).map(|amount| amount + 53)
                            }
                        } else {
                            None
                        },
                        // Drain
                        if new_params.player_mana >= 73 {
                            let mut choice_params = new_params;
                            choice_params.player_mana -= 73;
                            choice_params.boss_hit_points -= 2;
                            choice_params.player_hit_points += 2;
                            if choice_params.boss_hit_points <= 0 {
                                Some(73)
                            } else {
                                min_mana(choice_params, table).map(|amount| amount + 73)
                            }
                        } else {
                            None
                        },
                        // Shield
                        if new_params.shield_timer == 0 && new_params.player_mana >= 113 {
                            let mut choice_params = new_params;
                            choice_params.player_mana -= 113;
                            choice_params.shield_timer = 6;
                            min_mana(choice_params, table).map(|amount| amount + 113)
                        } else {
                            None
                        },
                        // Poison
                        if new_params.poison_timer == 0 && new_params.player_mana >= 173 {
                            let mut choice_params = new_params;
                            choice_params.player_mana -= 173;
                            choice_params.poison_timer = 6;
                            min_mana(choice_params, table).map(|amount| amount + 173)
                        } else {
                            None
                        },
                        // Recharge
                        if new_params.recharge_timer == 0 && new_params.player_mana >= 229 {
                            let mut choice_params = new_params;
                            choice_params.player_mana -= 229;
                            choice_params.recharge_timer = 5;
                            min_mana(choice_params, table).map(|amount| amount + 229)
                        } else {
                            None
                        },
                    ]
                    .iter()
                    .filter_map(|&amount| amount)
                    .min()
                }
            } else {
                new_params.player_hit_points -= (new_params.boss_damage - armor).max(1);
                if new_params.player_hit_points <= 0 {
                    None
                } else {
                    new_params.player_turn = true;
                    min_mana(new_params, table)
                }
            };

            table.insert(params, result);
            Some(result)
        })
        .unwrap()
}
