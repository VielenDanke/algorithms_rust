use std::cmp;

pub fn min_operations(s: String) -> i32 {
    let (n, mut count) = (s.len(), 0);
    if n == 1 {
        return 0;
    }
    for (i, c) in s.chars().enumerate() {
        let is_even = i % 2 == 0;

        if (is_even && c == '1') || (!is_even && c == '0') {
            count += 1;
        }
    }
    cmp::min(count, n as i32 - count)
}
