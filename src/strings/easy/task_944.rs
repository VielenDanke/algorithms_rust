pub struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let mut result = 0;

        'l: for i in 0..strs[0].len() {
            let mut prev = 0;
            for str in strs.iter() {
                let str_bytes = str.as_bytes();

                if str_bytes[i] < prev {
                    result += 1;
                    continue 'l;
                }
                prev = str_bytes[i];
            }
        }

        result
    }
}
