use std::collections::HashSet;

#[derive(Debug)]
pub struct FoodList {
    pub food: Vec<Food>,
    pub all_ingredients: HashSet<Ingredient>,
    pub all_allergens: HashSet<Allergen>
}

impl FoodList {
    pub fn new(food: Vec<Food>) -> FoodList {
        let mut all_ingredients = HashSet::new();
        let mut all_allergens = HashSet::new();

        for f in food.iter() {
            for ingredient in f.ingredients.iter() {
                all_ingredients.insert(ingredient.clone());
            }
            
            for allergen in f.allergens.iter() {
                all_allergens.insert(allergen.clone());
            }
        }

        FoodList { food, all_ingredients, all_allergens }
    }
}

#[derive(Debug)]
pub struct Food {
    pub ingredients: HashSet<Ingredient>,
    pub allergens: HashSet<Allergen>
}

impl Food {
    pub fn new(ingredients: HashSet<Ingredient>, allergens: HashSet<Allergen>) -> Food {
        Food { ingredients, allergens }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ingredient {
    pub name: String
}

impl Ingredient {
    pub fn new(name: String) -> Ingredient {
        Ingredient { name }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Allergen {
    pub name: String
}

impl Allergen {
    pub fn new(name: String) -> Allergen {
        Allergen { name }
    }
}