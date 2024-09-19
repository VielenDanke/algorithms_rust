pub struct Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let mut res = Vec::new();

        for (i, ch) in expression.chars().enumerate() {
            if ch == '+' || ch == '-' || ch == '*' {
                let s1 = Solution::diff_ways_to_compute(expression[0..i].to_string());
                let s2 = Solution::diff_ways_to_compute(expression[i+1..].to_string());
                for &a in s1.iter() {
                    for &b in s2.iter() {
                        match ch {
                            '+' => res.push(a + b),
                            '-' => res.push(a - b),
                            '*' => res.push(a * b),
                            _ => (),
                        }
                    }
                }
            }
        }
        if res.is_empty() {
            res.push(expression.parse().unwrap());
        }
        res
    }
}