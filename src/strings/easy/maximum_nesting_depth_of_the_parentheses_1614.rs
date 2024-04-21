struct Solution {}

impl Solution {
    pub fn max_depth(s: String) -> i32 {
        let (mut counter, mut result, left_bracket, right_bracket) = (0, 0, '(', ')');
        s.chars()
            .into_iter()
            .for_each(|c| {
                if c == left_bracket { counter += 1; } else if c == right_bracket { counter -= 1; }
                result = result.max(counter);
            });
        result
    }
}
