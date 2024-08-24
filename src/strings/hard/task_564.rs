pub struct Solution;

impl Solution {
    pub fn nearest_palindromic(n: String) -> String {
        let number = n.parse::<i64>().unwrap();
        if number <= 10 {
            return (number - 1).to_string();
        }
        if number == 11 {
            return (number - 2).to_string();
        }
        let m = n.len();

        let left_half = n[..(m + 1) / 2].parse::<i64>().unwrap();

        let mut palindrome_candidates = vec![
            Self::generate_palindrome_from_left(left_half - 1, m % 2 == 0),
            Self::generate_palindrome_from_left(left_half, m % 2 == 0),
            Self::generate_palindrome_from_left(left_half + 1, m % 2 == 0),
            10i64.pow((m - 1) as u32) - 1,
            10i64.pow(m as u32) + 1
        ];
        let mut nearest_palindrome = 0;
        let mut min_difference = i64::MAX;

        for &candidate in palindrome_candidates.iter() {
            if candidate == number {
                continue;
            }
            let diff = (candidate - number).abs();

            if diff < min_difference || (diff == min_difference && candidate < nearest_palindrome) {
                min_difference = diff;
                nearest_palindrome = candidate;
            }
        }
        nearest_palindrome.to_string()
    }

    fn generate_palindrome_from_left(mut left_half: i64, is_even_length: bool) -> i64 {
        let mut palindrome = left_half;
        if !is_even_length {
            left_half /= 10;
        }
        while left_half > 0 {
            palindrome = palindrome * 10 + left_half % 10;
            left_half /= 10;
        }
        palindrome
    }
}