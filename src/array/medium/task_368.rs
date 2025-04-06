pub struct Solution;

impl Solution {
    pub fn largest_divisible_subset(nums: Vec<i32>) -> Vec<i32> {
        fn all_subsets(
            result: &mut Vec<Vec<i32>>,
            temp: &mut Vec<i32>,
            nums: &Vec<i32>,
            idx: usize,
        ) {
            if !temp.is_empty() && (result.is_empty() || result[0].len() < temp.len()) {
                let mut is_found = true;
                'outer: for i in 0..temp.len() {
                    for j in i + 1..temp.len() {
                        if temp[i] % temp[j] != 0 && temp[j] % temp[i] != 0 {
                            is_found = false;
                            break 'outer;
                        }
                    }
                }
                if is_found {
                    if result.is_empty() {
                        result.push(temp.clone());
                    } else if result[0].len() < temp.len() {
                        result[0] = temp.clone();
                    }
                }
            }
            for i in idx..nums.len() {
                temp.push(nums[i]);
                all_subsets(result, temp, nums, i + 1);
                temp.pop();
            }
        }
        let mut subsets = vec![];

        all_subsets(&mut subsets, &mut Vec::new(), &nums, 0);

        subsets[0].clone()
    }

    pub fn largest_divisible_subset_dp(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut res = vec![Vec::new(); nums.len()];
        
        for i in 0..nums.len() {
            res[i].push(nums[i]);
        }
        for i in 1..nums.len() {
            for j in 0..i {
                if (nums[i] % nums[j] == 0) && (res[i].len() < res[j].len() + 1) {
                    let mut tmp = res[j].to_vec();
                    tmp.push(nums[i]);
                    res[i] = tmp;
                }
            }
        }

        let (mut max_len, mut max_pos) = (0, 0);
        
        for i in 0..res.len() {
            if res[i].len() > max_len {
                max_len = res[i].len();
                max_pos = i;
            }
        }
        res[max_pos].to_vec()
    }
}
