pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let m = 1001;

        let mut nums_1_counter = vec![0; m];
        let mut nums_2_counter = vec![0; m];

        for &num in nums1.iter() {
            nums_1_counter[num as usize] += 1;
        }

        for &num in nums2.iter() {
            nums_2_counter[num as usize] += 1;
        }

        let mut result = Vec::new();

        for i in 0..m {
            let num_to_push = i as i32;
            if nums_1_counter[i] > 0 && nums_2_counter[i] > 0 {
                let min_number_to_push = nums_1_counter[i].min(nums_2_counter[i]) as usize;
                for _ in 0..min_number_to_push {
                    result.push(num_to_push);
                }
            }
        }

        result
    }
}
