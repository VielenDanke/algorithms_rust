pub struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let m = 1001;

        let mut nums_1_counter = nums1.iter().fold(vec![0; m], |mut arr, &num| {
            arr[num as usize] += 1;
            arr
        });

        let nums_2_counter = nums2.iter().fold(vec![0; m], |mut arr, &num| {
            arr[num as usize] += 1;
            arr
        });

        nums_1_counter
            .iter()
            .zip(nums_2_counter)
            .enumerate()
            .flat_map(|(idx, zipped)| {
                let mut temp = Vec::new();
                if *zipped.0 > 0 && zipped.1 > 0 {
                    let min_number_to_push = *zipped.0.min(&zipped.1) as usize;
                    for _ in 0..min_number_to_push {
                        temp.push(idx);
                    }
                }
                temp
            })
            .map(|v| v as i32)
            .collect::<Vec<i32>>()
    }
}
