pub struct Solution;

impl Solution {
    pub fn num_sub(s: String) -> i32 {
        let modulus: i64 = 1_000_000_007;
        let mut consecutive = 0;
        let mut result = 0;

        for ch in s.chars() {
            if ch == '1' {
                consecutive += 1;
            } else {
                consecutive = 0;
            }
            result = (result + consecutive) % modulus;
        }
        result as i32
    }

    pub fn num_sub_brute_force(s: String) -> i32 {
        let n = s.len();
        let chars: Vec<char> = s.chars().collect();
        let modulus: i64 = 1_000_000_007;

        let mut pre_ones = vec![0; n + 1];

        for i in 0..n {
            pre_ones[i + 1] = pre_ones[i];
            if chars[i] == '1' {
                pre_ones[i + 1] += 1;
            }
        }

        let mut result: i64 = 0;

        for i in 0..n {
            for j in i..n {
                let length: i64 = (j - i + 1) as i64;

                let ones_count: i64 = pre_ones[j + 1] - pre_ones[i];

                if ones_count == length {
                    result = (result + 1) % modulus;
                }

                if ones_count < length {
                    break;
                }
            }
        }

        result as i32
    }
}