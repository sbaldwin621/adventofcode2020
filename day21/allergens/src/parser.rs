use std::collections::HashSet;

use nom::bytes::complete::{tag, take_while1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{delimited, tuple};
use nom::IResult;
use nom::character::complete::{line_ending, space0, space1};
use nom::error::ParseError;

use crate::food::{Allergen, Food, FoodList, Ingredient};

pub fn food_list(input: &str) -> IResult<&str, FoodList> {
    let (input, foods) = separated_list1(line_ending, food)(input)?;

    Ok((input, FoodList::new(foods)))
}

// mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
fn food(input: &str) -> IResult<&str, Food> {
    let (input, (ingredients, allergens)): (&str, (Vec<Ingredient>, Vec<Allergen>)) =
        tuple((ws(ingredient_list), ws(allergen_list)))(input)?;
    
    let ingredients = ingredients.into_iter().collect::<HashSet<_>>();
    let allergens = allergens.into_iter().collect::<HashSet<_>>();

    let food = Food::new(ingredients, allergens);

    Ok((input, food))
}

fn ingredient_list(input: &str) -> IResult<&str, Vec<Ingredient>> {
    separated_list1(space1, ingredient)(input)
}

fn ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(word, |s| Ingredient::new(s.to_string()))(input)
}

fn allergen_list(input: &str) -> IResult<&str, Vec<Allergen>> {
    delimited(
        tag("(contains "),
        separated_list1(tag(","), ws(allergen)),
        tag(")")
    )(input)
}

fn allergen(input: &str) -> IResult<&str, Allergen> {
    map(word, |s| Allergen::new(s.to_string()))(input)
}

fn word(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphabetic())(input)
}

fn ws<'a, F: 'a, O, E: ParseError<&'a str>>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
  where
  F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
  delimited(
    space0,
    inner,
    space0
  )
}
