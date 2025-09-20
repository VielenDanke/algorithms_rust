use std::cmp::Reverse;
use std::collections::{BTreeMap, BinaryHeap, HashMap};

struct FoodRatings {
    food_to_rating: HashMap<String, i32>,
    food_to_cuisine: HashMap<String, String>,
    cuisine_food_rating: HashMap<String, BTreeMap<i32, BinaryHeap<Reverse<String>>>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut food_to_rating = HashMap::new();
        let mut food_to_cuisine = HashMap::new();
        let mut cuisine_food_rating: HashMap<String, BTreeMap<i32, BinaryHeap<Reverse<String>>>> =
            HashMap::new();

        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];

            food_to_rating.insert(food.clone(), rating);
            food_to_cuisine.insert(food.clone(), cuisine.clone());

            let btree_map = cuisine_food_rating
                .entry(cuisine)
                .or_insert_with(BTreeMap::new);
            let binary_heap = btree_map.entry(rating).or_insert_with(BinaryHeap::new);
            binary_heap.push(Reverse(food));
        }

        FoodRatings {
            food_to_rating,
            food_to_cuisine,
            cuisine_food_rating,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let old_rating = self.food_to_rating.remove(&food).unwrap();
        self.food_to_rating.insert(food.clone(), new_rating);

        let cuisine = self.food_to_cuisine.get(&food).unwrap().clone();

        let btree_map = self.cuisine_food_rating.get_mut(&cuisine).unwrap();

        let heap = btree_map.get_mut(&old_rating).unwrap();

        // This is a "lazy deletion" strategy. Instead of a hard remove,
        // we just push the new item and deal with stale entries later.
        // This is necessary because BinaryHeap doesn't support an efficient remove.
        // The Java solution's `remove` on a PriorityQueue is not standard and
        // has a O(N) complexity for an arbitrary element, which is what we mimic here.

        if let Some(heap_with_old_rating) = btree_map.get_mut(&old_rating) {
            let mut temp_heap: Vec<Reverse<String>> = heap_with_old_rating.drain().collect();
            temp_heap.retain(|x| x.0 != food);
            for item in temp_heap {
                heap_with_old_rating.push(item);
            }

            if heap_with_old_rating.is_empty() {
                btree_map.remove(&old_rating);
            }
        }

        let new_heap = btree_map.entry(new_rating).or_insert_with(BinaryHeap::new);
        new_heap.push(Reverse(food));
    }

    fn highest_rated(&mut self, cuisine: String) -> String {
        let btree_map = self.cuisine_food_rating.get_mut(&cuisine).unwrap();

        loop {
            let (rating, heap) = btree_map.iter_mut().rev().next().unwrap();
            let top_food_rev = heap.peek().unwrap();
            let top_food_name = &top_food_rev.0;
            let current_rating = self.food_to_rating.get(top_food_name).unwrap();

            if *current_rating == *rating {
                return top_food_name.clone();
            } else {
                // The peeked item is stale, so we pop it.
                // This is now allowed because 'heap' is a mutable reference.
                let _ = heap.pop().unwrap();
            }
        }
    }
}
