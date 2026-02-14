use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let n = s_bytes.len();
        let mut result = 0;

        fn is_valid(arr: &[u8]) -> bool {
            ((arr[0] == arr[1]) && arr[2] == 0)
                || ((arr[1] == arr[2]) && arr[0] == 0)
                || ((arr[0] == arr[2]) && arr[1] == 0)
                || (arr[0] == arr[1] && arr[1] == arr[2])
                || (arr[0] == 0 && arr[1] == 0 && arr[2] != 0)
                || (arr[0] == 0 && arr[1] != 0 && arr[2] == 0)
                || (arr[0] != 0 && arr[1] == 0 && arr[2] == 0)
        }

        for i in 0..n {
            let mut arr = vec![0; 3];
            for j in i..n {
                let idx = (s_bytes[j] - b'a') as usize;
                arr[idx] += 1;
                if is_valid(&arr) {
                    result = result.max(j - i + 1);
                }
            }
        }

        result as i32
    }

    pub fn longest_balanced_faster(s: String) -> i32 {
        let s_bytes = s.as_bytes();
        let mut max_len = 0;

        // There are 7 possible non-empty subsets of {'a', 'b', 'c'}
        // {"a"}, {"b"}, {"c"}, {"a","b"}, {"a","c"}, {"b","c"}, {"a","b","c"}
        let subsets = vec![
            vec![b'a'],
            vec![b'b'],
            vec![b'c'],
            vec![b'a', b'b'],
            vec![b'a', b'c'],
            vec![b'b', b'c'],
            vec![b'a', b'b', b'c'],
        ];

        for subset in subsets {
            max_len = max_len.max(Self::solve_for_subset(s_bytes, &subset));
        }

        max_len
    }

    fn solve_for_subset(s: &[u8], subset: &[u8]) -> i32 {
        let mut map = HashMap::new();
        // Base case: 0 difference at index -1
        map.insert(vec![0; subset.len() - 1], -1);

        let mut counts = vec![0; subset.len()];
        let mut current_max = 0;

        for (i, &char_byte) in s.iter().enumerate() {
            // 1. Update counts only if char is in our target subset
            if let Some(pos) = subset.iter().position(|&x| x == char_byte) {
                counts[pos] += 1;
            } else {
                // 2. If we hit a character NOT in our subset,
                // this specific subset-window is broken. Reset.
                map.clear();
                counts = vec![0; subset.len()];
                map.insert(vec![0; subset.len() - 1], i as i32);
                continue;
            }

            // 3. Calculate relative differences
            // For subset {a, b, c}, we store (count_a - count_b, count_b - count_c)
            let mut diffs = Vec::with_capacity(subset.len() - 1);
            for j in 0..subset.len() - 1 {
                diffs.push(counts[j] - counts[j + 1]);
            }

            // 4. If we've seen this difference before, it means all characters
            // in the subset increased by the same amount.
            if let Some(&prev_idx) = map.get(&diffs) {
                // Ensure all characters in the subset have appeared at least once
                if counts.iter().all(|&c| c > 0) {
                    current_max = current_max.max(i as i32 - prev_idx);
                }
            } else {
                map.insert(diffs, i as i32);
            }
        }
        current_max
    }
}
