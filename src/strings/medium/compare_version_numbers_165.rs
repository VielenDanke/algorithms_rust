pub struct Solution;

impl Solution {
    pub fn compare_version(version1: String, version2: String) -> i32 {
        fn split_string(version: String) -> Vec<i32> {
            version.split(".")
                .into_iter().map(|v| {
                v.to_string().parse::<i32>().unwrap()
            }).collect::<Vec<i32>>()
        }
        let (versions_1, versions_2) = (split_string(version1), split_string(version2));

        let (mut left, mut right) = (0, 0);

        while left < versions_1.len() && right < versions_2.len() {
            if versions_1[left] != versions_2[right] {
                return if versions_1[left] < versions_2[right] {
                    -1
                } else {
                    1
                }
            }
            left += 1;
            right += 1;
        }
        while left < versions_1.len() {
            if versions_1[left] > 0 {
                return 1;
            }
            left += 1;
        }
        while right < versions_2.len() {
            if versions_2[right] > 0 {
                return -1;
            }
            right += 1;
        }
        0
    }
}
