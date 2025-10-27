pub struct Solution;

impl Solution {
    pub fn next_beautiful_number(n: i32) -> i32 {
        fn is_balance(mut x: i32) -> bool {
            let mut count = vec![0; 10];

            while x > 0 {
                count[x as usize % 10] += 1;
                x /= 10;
            }
            for i in 0..10 {
                if count[i] > 0 && count[i] != i {
                    return false;
                }
            }
            true
        }

        for i in n + 1..1224445 {
            if is_balance(i) {
                return i;
            }
        }
        -1
    }
}