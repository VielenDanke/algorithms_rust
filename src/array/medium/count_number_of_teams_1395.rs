pub struct Solution;

impl Solution {
    pub fn num_teams(rating: Vec<i32>) -> i32 {
        let m = rating.len();
        let mut increasing_cache = vec![vec![0; 4]; m];
        let mut decreasing_cache = vec![vec![0; 4]; m];

        for i in 0..m {
            increasing_cache[i][1] = 1;
            decreasing_cache[i][1] = 1;
        }

        for count in 2..=3 {
            for i in 0..m {
                for j in i + 1..m {
                    if rating[j] > rating[i] {
                        increasing_cache[j][count] += increasing_cache[i][count - 1];
                    }
                    if rating[j] < rating[i] {
                        decreasing_cache[j][count] += decreasing_cache[i][count - 1];
                    }
                }
            }
        }

        (0..m).into_iter().map(|i| increasing_cache[i][3] + decreasing_cache[i][3]).sum()
    }

    pub fn num_teams_brute_force(rating: Vec<i32>) -> i32 {
        // pick up 3 indicies such
        // r[i] < r[j] < r[k] || r[i] > r[j] > r[k]
        let m = rating.len();
        let mut result = 0;
        for i in 0..m {
            for j in i + 1..m {
                for k in j + 1..m {
                    if rating[i] < rating[j] && rating[j] < rating[k] {
                        result += 1;
                    } else if rating[i] > rating[j] && rating[j] > rating[k] {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}
