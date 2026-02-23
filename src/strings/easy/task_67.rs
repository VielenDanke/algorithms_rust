pub struct Solution;

impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut result = String::new();
        let a_bytes = a.as_bytes();
        let b_bytes = b.as_bytes();

        let mut i = a_bytes.len() as i32 - 1;
        let mut j = b_bytes.len() as i32 - 1;
        let mut carry = 0;

        // Loop as long as there are digits left in either string OR a remaining carry
        while i >= 0 || j >= 0 || carry > 0 {
            let mut sum = carry;

            if i >= 0 {
                sum += (a_bytes[i as usize] - b'0') as u32;
                i -= 1;
            }
            if j >= 0 {
                sum += (b_bytes[j as usize] - b'0') as u32;
                j -= 1;
            }

            // The new bit is sum % 2, the new carry is sum / 2
            result.push(if sum % 2 == 1 { '1' } else { '0' });
            carry = sum / 2;
        }

        // Since we pushed bits from right to left, we must reverse the string
        result.chars().rev().collect()
    }

    pub fn add_binary_overflow(a: String, b: String) -> String {
        fn convert_to_number(binary_string: String) -> u128 {
            let binary_string_bytes = binary_string.as_bytes();
            let n = binary_string_bytes.len();
            let mut b_num = 0;

            for i in (0..n).rev() {
                if binary_string_bytes[i] == b'1' {
                    b_num += 2u128.pow((n - i - 1) as u32);
                }
            }
            b_num
        }

        fn manual_divide_by_two(mut n: u128) -> String {
            if n == 0 {
                return "0".to_string();
            }

            let mut bits = Vec::new();

            while n > 0 {
                bits.push((n % 2).to_string());
                n /= 2;
            }

            // The bits are collected in reverse order, so we flip them
            bits.into_iter().rev().collect()
        }

        manual_divide_by_two(convert_to_number(a) + convert_to_number(b))
    }
}
