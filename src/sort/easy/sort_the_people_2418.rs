use std::collections::BTreeMap;

pub struct Solution;

impl Solution {
    pub fn sort_people(mut names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut m = BTreeMap::new();

        heights
            .iter()
            .enumerate()
            .for_each(|(i, &height)| {
                m.insert(height, names[i].clone());
            });

        for i in 0..names.len() {
            names[i] = m.pop_last().unwrap().1;
        }

        names
    }

    pub fn sort_people_zipped(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
        let mut zipped = names.into_iter().zip(heights.into_iter()).collect::<Vec<(String, i32)>>();
        zipped.sort_unstable_by(|a, b| b.1.cmp(&a.1));
        zipped.into_iter().map(|v| v.0).collect::<Vec<String>>()
    }
}
