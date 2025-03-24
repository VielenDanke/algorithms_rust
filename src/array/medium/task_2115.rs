use std::collections::{HashMap, HashSet};

pub struct Solution;

impl Solution {
    pub fn find_all_recipes_faster(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut supplies_map: HashSet<&str> = supplies.iter().map(|s| s.as_str()).collect();
        let mut recipe_map: HashMap<&str, &Vec<String>> = recipes
            .iter()
            .map(|s| s.as_str())
            .zip(ingredients.iter())
            .collect();

        for recipe in &recipes {
            Self::search(recipe, &mut recipe_map, &mut supplies_map);
        }

        recipe_map.keys().map(|r| r.to_string()).collect()
    }

    fn search<'a>(
        recipe: &'a str,
        recipe_map: &mut HashMap<&'a str, &'a Vec<String>>,
        supplies: &mut HashSet<&'a str>,
    ) {
        if supplies.contains(recipe) {
            return;
        }

        let Some(ingredient_list) = recipe_map.remove(recipe) else {
            return;
        };

        for ingredient in ingredient_list {
            if supplies.contains(ingredient.as_str()) {
                continue;
            }

            Self::search(ingredient, recipe_map, supplies);
            if !recipe_map.contains_key(ingredient.as_str()) {
                return;
            }
        }

        recipe_map.insert(recipe, ingredient_list);
        supplies.insert(recipe);
    }
    
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut available: HashSet<String> = supplies.into_iter().collect();
        let needs: HashMap<String, Vec<String>> = recipes
            .iter()
            .cloned()
            .zip(ingredients.into_iter())
            .collect();

        let mut updated = true;
        while updated {
            updated = false;
            for r in &recipes {
                if available.contains(r) {
                    continue;
                }

                if let Some(ing_list) = needs.get(r) {
                    if ing_list.iter().all(|ing| available.contains(ing)) {
                        available.insert(r.clone());
                        updated = true;
                    }
                }
            }
        }

        recipes
            .into_iter()
            .filter(|r| available.contains(r))
            .collect()
    }
}
