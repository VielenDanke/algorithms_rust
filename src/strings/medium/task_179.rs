pub struct Solution;

impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut nums_str: Vec<String> = nums.into_iter().map(|n| n.to_string()).collect::<Vec<String>>();

        nums_str.sort_unstable_by(|s1, s2| {
            let iter1 = s1.bytes().chain(s2.bytes());
            let iter2 = s2.bytes().chain(s1.bytes());

            iter2.cmp(iter1)
        });
        if nums_str.iter().all(|s| s == "0") {
            return "0".into();
        }
        nums_str.into_iter().collect()
    }
}