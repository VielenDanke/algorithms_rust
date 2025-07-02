pub struct Solution;

impl Solution {
    /// Returns the minimum number of deletions needed so that the difference
    /// between the highest and any other character frequency is at most `k`.
    pub fn minimum_deletions(w: String, k: i32) -> i32 {
        let mut freq = [0; 26];

        // Count frequency of each lowercase letter
        for byte in w.bytes() {
            freq[(byte - b'a') as usize] += 1;
        }

        let mut min_deletions = i32::MAX;

        // Try making each letter the "maximum frequency" letter
        for target in 0..26 {
            let mut deletions = 0;

            for c in 0..26 {
                if freq[c] < freq[target] {
                    deletions += freq[c];
                } else {
                    // Reduce freq[c] so that it's at most freq[target] + k
                    deletions += (freq[c] - freq[target] - k).max(0);
                }
            }

            min_deletions = min_deletions.min(deletions);
        }

        min_deletions
    }
}