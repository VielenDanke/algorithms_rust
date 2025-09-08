use std::collections::HashSet;

pub struct Solution;

impl Solution {
    pub fn reordered_power_of2_faster(mut n: i32) -> bool {
        fn counter(mut n: u32) -> i32 {
            let mut result = 0;

            while n > 0 {
                result += 10_i32.pow(n % 10);
                n /= 10;
            }
            result
        }
        let c = counter(n as u32);

        for i in 0..31 {
            if counter(1 << i) == c {
                return true;
            }
        }
        false
    }

    pub fn reordered_power_of2(mut n: i32) -> bool {
        let mut init_num: i32 = 1;

        let mut counter = 10000;

        let mut result = HashSet::new();

        while counter > 0 {
            let mut clone_init_num = init_num;
            let mut temp_vec = vec![];
            while clone_init_num > 0 {
                temp_vec.push(clone_init_num % 10);
                clone_init_num /= 10;
            }
            temp_vec.sort_unstable();

            result.insert(temp_vec.clone());

            init_num *= 2;

            counter -= 1;
        }
        let mut temp_vec = vec![];

        while n > 0 {
            temp_vec.push(n % 10);
            n /= 10;
        }
        temp_vec.sort_unstable();
        
        result.contains(&temp_vec)
    }
}
