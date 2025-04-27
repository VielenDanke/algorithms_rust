use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut counter = 0;

        for i in 0..n {
            for j in i + 1..n {
                if nums[i] == nums[j] && (i * j) as i32 % k == 0 {
                    counter += 1;
                }
            }
        }
        counter
    }

    pub fn count_pairs_faster(nums: Vec<i32>, k: i32) -> i32 {
        fn gcd(mut a: i32, mut b: i32) -> i32 {
            while b != 0 {
                let t = b;
                b = a % b;
                a = t;
            }
            a
        }
        let mut pairs = 0;
        let mut mpp: HashMap<i32, Vec<usize>> = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            mpp.entry(num).or_default().push(i);
        }

        let mut divisors = vec![];
        
        let mut d = 1;
        
        while d * d <= k {
            if k % d == 0 {
                divisors.push(d);
                if d != k / d {
                    divisors.push(k / d);
                }
            }
            d += 1;
        }

        for vec in mpp.values() {
            let mut internal_counter: HashMap<i32, i32> = HashMap::new();
            
            for &i in vec {
                let gcdd = gcd(i as i32, k);
                let need = k / gcdd;
                pairs += *internal_counter.get(&need).unwrap_or(&0);
                for &d in &divisors {
                    if (i as i32) % d == 0 {
                        *internal_counter.entry(d).or_insert(0) += 1;
                    }
                }
            }
        }

        pairs
    }
}
