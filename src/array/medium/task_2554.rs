use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn max_count(mut banned: Vec<i32>, n: i32, max_sum: i32) -> i32 {
        let mut arr = Vec::new();

        for i in 1..=n {
            arr.push(i);
        }
        banned = banned.into_iter().collect::<HashSet<i32>>().into_iter().collect::<Vec<i32>>();
        banned.sort_unstable();

        let mut banned_idx = 0;
        let mut current_sum = 0;
        let mut total = 0;

        for i in 0..n as usize {
            if banned_idx < banned.len() && arr[i] == banned[banned_idx] {
                banned_idx += 1;
            } else {
                if current_sum + arr[i] <= max_sum {
                    current_sum += arr[i];
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