pub struct Solution;

impl Solution {
    pub fn largest_combination(candidates: Vec<i32>) -> i32 {
        let mut bit_counts = vec![0; 32];

        // Count bits for each number
        for &num in candidates.iter() {
            Self::count_bits(num, &mut bit_counts);
        }

        // Find maximum count
        *bit_counts.iter().max().unwrap()
    }

    fn count_bits(mut num: i32, bit_counts: &mut Vec<i32>) {
        for i in 0..32 {
            if (num & (1 << i)) != 0 {
                bit_counts[i] += 1;
            }
        }
    }
}