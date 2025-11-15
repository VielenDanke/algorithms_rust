pub struct Solution;

impl Solution {
    /// Calculates the maximum number of operations on the string.
    ///
    /// The problem describes an operation where a '1' at index `i` can jump over
    /// one or more '0's at `i+1, i+2, ...` if `s[i] == '1'` and `s[i+1] == '0'`.
    /// The '1' stops before the next '1' or at the end of the string.
    ///
    /// Example: "1001" -> "0011" (1 operation)
    /// Example: "1100" -> "1001" (1 op) -> "0011" (1 op) = 2 operations
    ///
    /// Insight:
    /// The logic is based on "block merges". We track the count of '1's seen so far.
    /// When we transition from a '0' block to a '1' (a "01" pattern), it means
    /// the preceding block of '1's has "jumped" over the '0' block and merged
    /// with this new '1'. The number of operations is equal to the size of that
    /// preceding '1's block.
    ///
    /// We also handle the case where the string ends in '0' (e.g., "1100"),
    /// which means the final block of '1's performs its jumps.
    ///
    /// `s = "1001101"`
    /// `ones_count = 0`, `ops = 0`, `last_char = ' '`
    /// `c = '1'`: `last_char` is not '0'. `ones_count = 1`. `last_char = '1'`.
    /// `c = '0'`: `last_char = '0'`.
    /// `c = '0'`: `last_char = '0'`.
    /// `c = '1'`: `last_char` is '0'. `ops += ones_count` (ops=1). `ones_count = 2`. `last_char = '1'`.
    /// `c = '1'`: `last_char` is not '0'. `ones_count = 3`. `last_char = '1'`.
    /// `c = '0'`: `last_char = '0'`.
    /// `c = '1'`: `last_char` is '0'. `ops += ones_count` (ops=1+3=4). `ones_count = 4`. `last_char = '1'`.
    /// End loop.
    /// `last_char` is not '0'.
    /// Return `ops` (4).
    ///
    pub fn max_operations(s: String) -> i32 {
        let mut ops = 0;
        let mut ones_count = 0;
        let mut last_char = ' '; // Use a neutral start character

        // Iterate over the characters of the string
        for c in s.chars() {
            if c == '1' {
                if last_char == '0' {
                    // This '1' ends a '0' block.
                    // The preceding '1' block (of size ones_count)
                    // has now "merged" by jumping the '0' block.
                    ops += ones_count;
                }
                ones_count += 1;
                last_char = '1';
            } else {
                // c == '0'
                last_char = '0';
            }
        }

        // Handle trailing zeros
        // If the string ended with '0' (e.g., "1100"),
        // the last '1' block needs to "jump".
        if last_char == '0' {
            ops += ones_count;
        }

        // In Rust, the last expression is implicitly returned
        ops
    }
}
