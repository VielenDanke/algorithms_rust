pub struct Solution;

impl Solution {
    pub fn nth_ugly_number_brute_force(mut n: i32) -> i32 {
        let prime_factors = vec![5, 3, 2];
        let m = prime_factors.len();

        let mut answer = 1;
        let mut value = 1;

        while n > 0 {
            let mut current_value = value;
            for i in 0..m {
                while current_value % prime_factors[i] == 0 && current_value != prime_factors[i] {
                    current_value /= prime_factors[i];
                }
                if current_value == prime_factors[i] {
                    current_value = 1;
                    break;
                }
            }
            if current_value == 1 {
                n -= 1;
                answer = value;
            }
            value += 1;
        }
        answer
    }

    pub fn nth_ugly_number(mut n: i32) -> i32 {
        let mut ugly_sequence = vec![1];

        let (mut index2, mut index3, mut index5) = (0, 0, 0);
        let (mut next2, mut next3, mut next5) = (2, 3, 5);

        for _ in 1..n {
            let next_ugly = next2.min(next3).min(next5);
            ugly_sequence.push(next_ugly);

            if next_ugly == next2 {
                index2 += 1;
                next2 = ugly_sequence[index2] * 2;
            }
            if next_ugly == next3 {
                index3 += 1;
                next3 = ugly_sequence[index3] * 3;
            }
            if next_ugly == next5 {
                index5 += 1;
                next5 = ugly_sequence[index5] * 5;
            }
        }
        *ugly_sequence.last().unwrap()
    }
}