pub struct Solution;

impl Solution {
    pub fn num_rescue_boats(mut people: Vec<i32>, limit: i32) -> i32 {
        people.sort_unstable();

        let (mut left, mut right, mut result) = (0usize, people.len() - 1, 0);

        while left < right {
            if people[right] + people[left] <= limit {
                left += 1;
            }
            right -= 1;
            result += 1;
        }
        result + if left == right { 1 } else { 0 }
    }
}
