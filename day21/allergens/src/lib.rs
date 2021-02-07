use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt::Display;
use std::fs::{File, read_to_string};
use std::io::{self, BufRead};
use std::path::Path;

use config::Config;
use food::{Allergen, FoodList, Ingredient};
use parser::food_list;

pub mod config;
mod food;
mod parser;

pub fn run(config: Config) -> Result<u32, Box<dyn Error>> {
    let filename = config.filename;

    let input = read_to_string(filename)?;

    let (_, foods) = food_list(&input).map_err(|e| ApplicationError::ParseInputError(e.to_string()))?;

    // println!("{:?}", foods);

    count_nonallergens(&foods);

    Ok(0)
}

fn count_nonallergens(food_list: &FoodList) -> usize {
    let mut count = 0;

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

            if ingredient_set.len() == 1 {
                let ingredient = ingredient_set.into_iter().nth(0).unwrap();
                assigned_ingredients.insert(ingredient.clone());
                allergen_to_ingredient.insert(allergen.clone(), ingredient);
            } else if ingredient_set.len() > 1 {
                allergen_to_potential_ingredients.insert(allergen.clone(), ingredient_set);
            }
        }
    }

    println!("definite:  {:?}", allergen_to_ingredient);
    println!("potential: {:?}", allergen_to_potential_ingredients);

    for (allergen, mut ingredients) in allergen_to_potential_ingredients.iter_mut() {
        for ingredient in assigned_ingredients.iter() {
            ingredients.remove(ingredient);
        }
    }

    count
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
