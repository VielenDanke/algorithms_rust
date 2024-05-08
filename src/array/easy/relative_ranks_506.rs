use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn find_relative_ranks(scores: Vec<i32>) -> Vec<String> {
        let mut cloned_score = scores.clone();

        cloned_score.sort_unstable();

        let mut place = scores.len();

        let mut m = HashMap::new();

        for score in cloned_score {
            m.insert(score, place);
            place -= 1;
        }
        let mut result = vec!["".to_string(); scores.len()];

        for (i, v) in scores.iter().enumerate() {
            let mut place = m[v];

            result[i] = match place {
                1 => String::from("Gold Medal"),
                2 => String::from("Silver Medal"),
                3 => String::from("Bronze Medal"),
                _ => place.to_string(),
            }
        }
        result
    }

    pub fn find_relative_ranks_binary_search(scores: Vec<i32>) -> Vec<String> {
        let mut sorted_scores = scores.clone();
        sorted_scores.sort_unstable();
        let n = scores.len();

        let mut result = Vec::new();

        for score in scores {
            let idx = n - sorted_scores.binary_search(&score).unwrap();
            result.push(
                match idx {
                    1 => String::from("Gold Medal"),
                    2 => String::from("Silver Medal"),
                    3 => String::from("Bronze Medal"),
                    _ => idx.to_string()
                }
            );
        }
        result
    }
}
