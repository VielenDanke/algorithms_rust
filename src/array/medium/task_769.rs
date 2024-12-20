pub struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted_short(arr: Vec<i32>) -> i32 {
        let mut max   = 0;
        let mut count = 0;

        for (i, &num) in arr.iter().enumerate() {
            max = max.max(num);

            if max == i as i32 {
                count += 1;
            }
        }
        count
    }

    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        // find the lowest number
        // pick up a chunk
        // sort
        // check if it's sorted
        // if yes - pick the greatest number
        let n = arr.len();
        let mut expected_number = 0;
        let mut left = 0;
        let mut right = 0;
        let mut result = 0;
        let mut is_required_check = false;

        'outer: while right < n {
            if arr[right] == expected_number || is_required_check {
                let mut sub = arr[left..=right].to_vec();
                sub.sort_unstable();
                let mut start = expected_number;
                for j in 0..sub.len() {
                    if sub[j] != start {
                        is_required_check = true;
                        right += 1;
                        continue 'outer;
                    }
                    start += 1;
                }
                is_required_check = false;
                result += 1;
                expected_number = (right + 1) as i32;
                left = right + 1;
            }
            right += 1;
        }
        if result == 0 { 1 } else { result }
    }
}