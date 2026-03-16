pub struct Solution;

impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let bytes = s.as_bytes();

        // Use a vector to store the doubled string for the sliding window
        let mut s2 = Vec::with_capacity(2 * n);
        for _ in 0..2 {
            s2.extend_from_slice(bytes);
        }

        let mut diff1 = 0; // Flips needed for pattern starting with '0'
        let mut diff2 = 0; // Flips needed for pattern starting with '1'
        let mut min_flips = i32::MAX;

        // Sliding window of size n over the doubled string
        for i in 0..s2.len() {
            // Calculate expected characters for the two possible alternating patterns
            let target1 = if i % 2 == 0 { b'0' } else { b'1' };
            let target2 = if i % 2 == 0 { b'1' } else { b'0' };

            // Add new character's contribution to diffs
            if s2[i] != target1 {
                diff1 += 1;
            }
            if s2[i] != target2 {
                diff2 += 1;
            }

            // If the window exceeds size n, remove the leftmost character's contribution
            if i >= n {
                let prev_idx = i - n;
                let prev_target1 = if prev_idx % 2 == 0 { b'0' } else { b'1' };
                let prev_target2 = if prev_idx % 2 == 0 { b'1' } else { b'0' };

                if s2[prev_idx] != prev_target1 {
                    diff1 -= 1;
                }
                if s2[prev_idx] != prev_target2 {
                    diff2 -= 1;
                }
            }

            // Once we have a full window, update the global minimum
            if i >= n - 1 {
                min_flips = min_flips.min(diff1.min(diff2));
            }
        }

        min_flips
    }
}
