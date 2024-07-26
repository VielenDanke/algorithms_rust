pub struct Solution;

impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        // 5*10^4 + 4*10^3 + 3*10^2 + 2*10^1 + 1*10^0 = 54321
        let mut number_pairs = Vec::new();

        for &num in nums.iter() {
            let mut cloned_num = num.clone();
            let mut mapped_number = 0;
            let mut power = 1;
            if cloned_num == 0 {
                mapped_number = mapping[cloned_num as usize];
            } else {
                while cloned_num > 0 {
                    let last_number = cloned_num % 10;
                    let mapped_last_number = mapping[last_number as usize];
                    mapped_number += mapped_last_number * power;
                    power *= 10;
                }
            }
            number_pairs.push((num, mapped_number));
        }

        number_pairs.sort_unstable_by_key(|key| key.1);

        number_pairs.iter().map(|v| v.0).collect::<Vec<i32>>()
    }
}
