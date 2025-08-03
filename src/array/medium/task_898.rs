use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn subarray_bitwise_o_rs(arr: Vec<i32>) -> i32 {
        let mut window = arr.len();

        let mut res = HashSet::new();

        while window > 0 {
            let x = arr
                .windows(window)
                .into_iter()
                .map(|x| x.iter().fold(0, |acc, &x| acc | x))
                .collect::<HashSet<_>>();
            res.extend(x);
            window -= 1;
        }
        res.iter().count() as i32
    }

    pub fn subarray_bitwise_o_rs_faster(arr: Vec<i32>) -> i32 {
        let mut res: HashSet<i32> = HashSet::new();
        let mut current_ors = HashSet::new();

        for &num in arr.iter() {
            let mut next_ors = HashSet::from([num]);

            for &prev_or in current_ors.iter() {
                next_ors.insert(prev_or | num);
            }

            current_ors = next_ors;

            res.extend(current_ors.iter());
        }

        res.len() as i32
    }
}
