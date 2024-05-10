pub struct Solution;

impl Solution {
    pub fn maximum_happiness_sum(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        happiness.sort_unstable_by_key(|&x|-x);

        let mut result = 0i64;

        for (i, x) in happiness[..k as usize].iter().enumerate().map(|(i, &x)| (i as i64, x as i64)) {
            if x <= i {
                break;
            }
            result += x - i;
        }
        result
    }

    pub fn maximum_happiness_sum_more_readable(mut happiness: Vec<i32>, mut k: i32) -> i64 {
        happiness.sort_unstable();

        let (mut idx, mut decrementor, mut result) = (happiness.len() - 1, 0, 0i64);

        while idx >= 0 && k > 0 {
            let current_happiness = i32::max(happiness[idx] - decrementor, 0) as i64;
            if current_happiness == 0 {
                break;
            }
            result += current_happiness;
            decrementor += 1;
            k -= 1;
            if idx == 0 {
                break;
            }
            idx -= 1;
        }
        result
    }
}
