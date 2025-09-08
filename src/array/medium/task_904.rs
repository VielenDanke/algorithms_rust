/*
You are visiting a farm that has a single row of fruit trees arranged from left to right. The trees are represented by an integer array fruits where fruits[i] is the type of fruit the ith tree produces.

You want to collect as much fruit as possible. However, the owner has some strict rules that you must follow:

You only have two baskets, and each basket can only hold a single type of fruit. There is no limit on the amount of fruit each basket can hold.
Starting from any tree of your choice, you must pick exactly one fruit from every tree (including the start tree) while moving to the right. The picked fruits must fit in one of your baskets.
Once you reach a tree with fruit that cannot fit in your baskets, you must stop.
Given the integer array fruits, return the maximum number of fruits you can pick.
 */
use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut counter = HashMap::new();
        let max_available_buckets = 2;
        let mut max_fruits = 0;
        let mut left = 0;

        for right in 0..fruits.len() {
            *counter.entry(fruits[right]).or_insert(0) += 1;

            while counter.len() > max_available_buckets {
                let mut current_entry = counter.get_mut(&fruits[left]).unwrap();

                *current_entry -= 1;

                if *current_entry == 0 {
                    counter.remove(&fruits[left]);
                }

                left += 1;
            }

            max_fruits = max_fruits.max((right - left + 1) as i32);
        }

        max_fruits
    }
}
