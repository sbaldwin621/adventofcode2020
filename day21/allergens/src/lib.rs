use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use food::{Allergen, FoodList, Ingredient};
use nom::bitvec::index::BitPos;
use parser::food_list;

pub mod config;
mod food;
mod parser;

pub fn run(config: Config) -> Result<usize, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;

    let (_, foods) = food_list(&input).map_err(|e| ApplicationError::ParseInputError(e.to_string()))?;

    // println!("{:?}", foods);

    let allergen_to_ingredient = map_allergens(&foods);
    let ingredients_with_allergens = allergen_to_ingredient.values().map(|i| i.clone()).collect::<HashSet<_>>();
    let nonallergens = foods.all_ingredients.difference(&ingredients_with_allergens).map(|i| i.clone()).collect::<HashSet<_>>();

    let mut count = 0;
    for food in foods.food.iter() {
        count += food.ingredients.intersection(&nonallergens).count();
    }

    Ok(count)
}

fn map_allergens(food_list: &FoodList) -> HashMap<Allergen, Ingredient> {
    let mut allergen_to_ingredient = HashMap::new();
    let mut assigned_ingredients = HashSet::new();

    let mut allergen_to_potential_ingredients: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();

    // Gather list of possible matches for each allergen
    for food in food_list.food.iter() {
        for allergen in food.allergens.iter() {
            let ingredient_set: HashSet<Ingredient>;
            if let Some(ingredients_for_allergen) = allergen_to_potential_ingredients.get(allergen) {
                ingredient_set = ingredients_for_allergen.intersection(&food.ingredients).map(|i| i.clone()).collect::<HashSet<_>>();
            } else {
                ingredient_set = food.ingredients.clone();
            }

            allergen_to_potential_ingredients.insert(allergen.clone(), ingredient_set);
        }
    }

    let mut change_was_made = true;
    while change_was_made {
        change_was_made = false;

        for (allergen, ingredients) in allergen_to_potential_ingredients.iter_mut() {
            for ingredient in assigned_ingredients.iter() {
                if ingredients.remove(ingredient) {
                    change_was_made = true;
                }
            }
            
            if ingredients.len() == 1 {
                change_was_made = true;

                let ingredient = ingredients.iter().nth(0).unwrap().clone();
                ingredients.remove(&ingredient);
    
                assigned_ingredients.insert(ingredient.clone());
                allergen_to_ingredient.insert(allergen.clone(), ingredient);
            }
        }

        for allergin in allergen_to_ingredient.keys() {
            if let Some(_) = allergen_to_potential_ingredients.remove(allergin) {
                change_was_made = true;
            }
        }
    }

    println!("{:?}", allergen_to_ingredient);
    
    allergen_to_ingredient
}

#[derive(Debug)]
pub enum ApplicationError {
    ParseInputError(String)
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", match self {
            ApplicationError::ParseInputError(error) => error
        })
    }
}

impl Error for ApplicationError { }
