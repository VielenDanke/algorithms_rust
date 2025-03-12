pub struct Solution;

impl Solution {
    pub fn number_of_alternating_groups(mut colors: Vec<i32>, k: i32) -> i32 {
        let mut temp = colors.clone();
        temp.extend_from_slice(&colors[..(k as usize - 1)]);

        let mut count = 0;
        let mut left = 0;

        for right in 0..temp.len() {
            if right > 0 && temp[right] == temp[right - 1] {
                left = right;
            }

            if right - left + 1 >= k as usize {
                count += 1;
            }
        }

        count
    }
}
