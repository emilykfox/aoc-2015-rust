use std::{collections::HashMap, io::stdin};

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn main() {
    let re = regex::Regex::new(r#"^\w+: capacity (?<capacity>-?\d+), durability (?<durability>-?\d+), flavor (?<flavor>-?\d+), texture (?<texture>-?\d+), calories (?<calories>\d+)$"#).unwrap();

    let mut ingredients = Vec::<Ingredient>::new();
    for line in stdin().lines().map_while(Result::ok) {
        let captures = re.captures(&line).expect(&line);

        let ingredient = Ingredient {
            capacity: captures["capacity"].parse().unwrap(),
            durability: captures["durability"].parse().unwrap(),
            flavor: captures["flavor"].parse().unwrap(),
            texture: captures["texture"].parse().unwrap(),
            calories: captures["calories"].parse().unwrap(),
        };

        ingredients.push(ingredient);
    }

    let best_cookie = best_recipe(&ingredients, &mut Vec::new(), None).unwrap();

    println!("Part 1: {best_cookie}");

    let best_diet_cookie = best_recipe(&ingredients, &mut Vec::new(), Some(500)).unwrap();

    println!("Part 2: {best_diet_cookie}");
}

fn best_recipe(
    ingredients: &Vec<Ingredient>,
    amounts: &mut Vec<i32>,
    calorie_goal: Option<i32>,
) -> Option<i32> {
    let remaining_room = 100 - amounts.iter().sum::<i32>();
    if amounts.len() == ingredients.len() {
        if remaining_room == 0 {
            let total_calories = ingredients
                .iter()
                .zip(amounts.iter())
                .map(|(ingredient, allocation)| allocation * ingredient.calories)
                .sum::<i32>()
                .max(0);
            if calorie_goal.is_none_or(|goal| goal == total_calories) {
                let total_capacity = ingredients
                    .iter()
                    .zip(amounts.iter())
                    .map(|(ingredient, allocation)| allocation * ingredient.capacity)
                    .sum::<i32>()
                    .max(0);
                let total_durability = ingredients
                    .iter()
                    .zip(amounts.iter())
                    .map(|(ingredient, allocation)| allocation * ingredient.durability)
                    .sum::<i32>()
                    .max(0);
                let total_flavor = ingredients
                    .iter()
                    .zip(amounts.iter())
                    .map(|(ingredient, allocation)| allocation * ingredient.flavor)
                    .sum::<i32>()
                    .max(0);
                let total_texture = ingredients
                    .iter()
                    .zip(amounts.iter())
                    .map(|(ingredient, allocation)| allocation * ingredient.texture)
                    .sum::<i32>()
                    .max(0);
                let score = total_capacity * total_durability * total_flavor * total_texture;
                Some(score)
            } else {
                None
            }
        } else {
            None
        }
    } else {
        let mut best_score: Option<i32> = None;
        for amount in 0..=remaining_room {
            amounts.push(amount);
            let score = best_recipe(ingredients, amounts, calorie_goal);
            if let Some(score) = score {
                best_score =
                    best_score.map_or(Some(score), |best_score| Some(best_score.max(score)));
            }
            _ = amounts.pop();
        }
        best_score
    }
}
