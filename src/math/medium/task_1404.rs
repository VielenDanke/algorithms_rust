pub struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let mut operations = 0;
        let mut carry = 0;

        // We iterate from right to left, skipping the first character (index 0)
        // just like the i > 0 condition in your Java loop.
        for c in s.chars().rev().take(s.len() - 1) {
            // Convert char to digit ('0' -> 0, '1' -> 1) and add carry
            let digit = (c as i32 - '0' as i32) + carry;

            if digit % 2 == 1 {
                // If it's odd: we add 1 (to make it even) then divide by 2.
                // That's 2 operations.
                operations += 2;
                carry = 1;
            } else {
                // If it's even: just divide by 2.
                // That's 1 operation.
                operations += 1;
            }
        }

        operations + carry
    }

    pub fn num_steps_unwrap_error_overflow(s: String) -> i32 {
        let mut num = u64::from_str_radix(&s, 2).unwrap();
        let mut steps = 0;

        while num > 1 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num += 1;
            }
            steps += 1;
        }
        steps
    }
}