use std::collections::HashSet;

pub struct Solution;

impl Solution {
    // beat 100%
    pub fn max_count(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut total = 0;
        let mut sum = 0;
        let max_possible_size = (*banned.iter().max().unwrap() + 1).max(n + 1) as usize;

        let mut banned_calc = vec![0; max_possible_size];

        for &v in banned.iter() {
            banned_calc[v as usize] += 1;
        }
        for i in 1..=n {
            if banned_calc[i as usize] == 0 {
                if sum + i <= max_sum {
                    sum += i;
                    total += 1;
                } else {
                    return total;
                }
            }
        }
        total
    }

    pub fn max_count_only_set(banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut set = banned.into_iter().collect::<HashSet<i32>>();
        let mut sum = 0;
        let mut total = 0;
        for i in 1..=n {
            if sum + i > max_sum {
                break;
            }
            if set.contains(&i) {
                continue;
            }
            sum += i;
            total += 1;
        }
        total
    }
}