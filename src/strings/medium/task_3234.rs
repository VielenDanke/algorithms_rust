pub struct Solution;

impl Solution {
    fn is_dominant(sub_bytes: &[u8]) -> bool {
        let mut zeros = 0;
        let mut ones = 0;

        for i in 0..sub_bytes.len() {
            if sub_bytes[i] == b'0' {
                zeros += 1;
            } else {
                ones += 1;
            }
        }
        ones >= zeros * zeros
    }

    pub fn number_of_substrings_2_prefix_sum(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();
        if n == 0 { return 0; }

        // --- 1. Correct Prefix Sum Calculation ---
        // We'll use (n+1) size for easier 1-based indexing
        // pre_zeros[i] = count of '0's in the first i characters (s[0..i-1])
        let mut pre_zeros = vec![0; n + 1];
        let mut pre_ones = vec![0; n + 1];

        for i in 0..n {
            pre_zeros[i + 1] = pre_zeros[i] + if chars[i] == '0' { 1 } else { 0 };
            pre_ones[i + 1] = pre_ones[i] + if chars[i] == '1' { 1 } else { 0 };
        }

        // Example: s = "010"
        // pre_zeros = [0, 1, 1, 2]
        // pre_ones  = [0, 0, 1, 1]

        let mut result = 0;

        // --- 2. Correct O(N^2) Loop Logic ---
        // Iterate over every possible start index `i`
        for i in 0..n {
            // Iterate over every possible end index `j`
            for j in i..n {

                // Get counts for the substring s[i..=j]
                // We use 1-based indices for the prefix arrays
                let zeros = pre_zeros[j + 1] - pre_zeros[i];
                let ones = pre_ones[j + 1] - pre_ones[i];

                // Check the dominance condition
                if ones >= (zeros * zeros) {
                    result += 1;
                }
            }
        }
        result
    }

    pub fn number_of_substrings(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let n = chars.len();

        // `pre[i]` will store the 0-based index of the *start* of the contiguous block
        // of '1's that `chars[i-1]` belongs to.
        // Or, if `chars[i-1]` is '0', it stores the start of the '1's block *before* it,
        // or its own index if it follows another '0' or is at the start.
        // This allows us to "jump" from a '0' to the '0' before the previous '1's block.
        let mut pre = vec![-1; n + 1];
        for i in 0..n {
            if i == 0 || chars[i - 1] == '0' {
                // If we are at the start or see a '0', this position starts a new "block"
                // (either of a '0' or a new '1's sequence).
                pre[i + 1] = i as i32;
            } else {
                // If we see a '1', it belongs to the same block as the previous character.
                pre[i + 1] = pre[i];
            }
        }

        let mut res = 0i32; // This will store the total count of dominant substrings.

        // Iterate through every possible ENDING position `i` for a substring.
        // `i` is 1-based, so it represents the substring s[...i-1].
        for i in 1..=n {
            // `cnt0` will track the number of zeros in the substring we're checking.
            // We start with the character at the end position `i-1`.
            let mut cnt0 = if chars[i - 1] == '0' { 1 } else { 0 };

            // `j` is our "start" pointer. We will move `j` backward by "jumping"
            // from one '0' block to the previous '0' block using our `pre` array.
            let mut j = i as i32;

            // --- The Core Logic Loop ---
            // We jump `j` backward, adding one '0' to our count (`cnt0`) with each jump.
            // `j > 0`: Keep going until we've processed the whole string.
            // `(cnt0 * cnt0) as usize <= n`: This is a key optimization.
            // The condition is `cnt1 >= cnt0 * cnt0`. The max possible `cnt1` is `n`.
            // If `cnt0 * cnt0` is already > `n`, no substring with this many zeros
            // can ever be dominant, so we can stop this inner loop early.
            while j > 0 && (cnt0 * cnt0) as usize <= n {

                // --- Calculate counts for the current "jump" block ---
                // `i`: The 1-based end of our substring.
                // `pre[j as usize]`: The 0-based start of the current block.
                // `i - pre[j]`: The total length of the substring s[pre[j]..i].
                // `cnt0`: The number of zeros we've counted so far (from `i` back to `j`).
                // `cnt1`: Total length - zero count = one count.
                let cnt1 = (i as i32 - pre[j as usize]) - cnt0;

                // This is the problem's dominance condition: ones >= zeros^2
                if cnt0 * cnt0 <= cnt1 {

                    // If dominant, we add a *group* of valid substrings to the result.
                    // All these substrings end at `i-1`. Their start positions are
                    // within the current '1's block defined by `s[pre[j]..j]`.

                    // `j - pre[j as usize]`: This is the number of characters in the
                    // current block, which is the *max number of available start positions*.
                    // (e.g., if block is "111", length is 3. We can start at s[0], s[1], or s[2]).

                    // `cnt1 - cnt0 * cnt0 + 1`: This is the "surplus" of ones. It tells us
                    // how many ones we can afford to remove from the start of the
                    // substring while *still* remaining dominant. This is the
                    // *max number of valid start positions*.

                    // We add the minimum of the two. We can't start before the block
                    // (limited by `j - pre[j]`) and we can't start so late that the
                    // substring is no longer dominant (limited by `cnt1 - cnt0*cnt0 + 1`).
                    res += std::cmp::min(j - pre[j as usize], cnt1 - cnt0 * cnt0 + 1);
                }

                // --- Prepare for the next jump ---
                // Move `j` to the start of the *previous* block, effectively skipping
                // the block of '1's we just processed.
                j = pre[j as usize];

                // Since we've jumped *over* the '0' at index `j-1` (or the start),
                // we increment our zero count for the next iteration.
                cnt0 += 1;
            }
        }
        res
    }

    pub fn number_of_substrings_brute_force(s: String) -> i32 {
        let mut window = 1;
        let s_bytes = s.as_bytes();
        let mut result = 0;

        while window <= s_bytes.len() {
            for i in 0..s_bytes.len() {
                if i + window <= s_bytes.len() {
                    if Self::is_dominant(&s_bytes[i..i + window]) {
                        result += 1;
                    }
                }
            }

            window += 1;
        }
        result
    }
}
