pub struct Solution;

impl Solution {
    pub fn equal_substring(s: String, t: String, max_cost: i32) -> i32 {
        let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());

        let (mut left, mut right, mut temp, mut max_len) = (0, 0, 0, 0);

        while right < t.len() {
            temp += (t_bytes[right].abs_diff(s_bytes[right])) as i32;
            while left < right && temp > max_cost {
                temp -= (t_bytes[left].abs_diff(s_bytes[left])) as i32;
                left += 1;
            }
            if temp <= max_cost {
                max_len = max_len.max((right - left) as i32 + 1);
            }
            right += 1;
        }
        max_len
    }
}
