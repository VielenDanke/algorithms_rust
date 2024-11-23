pub struct Solution;

impl Solution {
    pub fn take_characters_faster(s: String, k: i32) -> i32 {
        let mut freq = vec![0; 3];
        let n = s.len();
        let s_bytes = s.as_bytes();

        for &c in s_bytes {
            freq[(c - b'a') as usize] += 1;
        }

        if freq[0] < k || freq[1] < k || freq[2] < k {
            return -1;
        }

        let mut curr = vec![0; 3];
        let mut max_len = 0;
        let mut left = 0;

        for right in 0..n {
            curr[(s_bytes[right] - b'a') as usize] += 1;

            while left <= right && (curr[0] > freq[0] - k ||
                curr[1] > freq[1] - k ||
                curr[2] > freq[2] - k) {
                curr[(s_bytes[left] - b'a') as usize] -= 1;
                left += 1;
            }

            max_len = max_len.max(right - left + 1);
        }

        (n - max_len) as i32
    }

    pub fn take_characters(s: String, k: i32) -> i32 {
        let mut bytes = s.as_bytes().clone();

        let left = 0;
        let right = bytes.len() - 1;

        let mut counter = Vec::new();

        for _ in 0..3 {
            counter.push(0);
        }

        Self::backtrack(bytes, left, right, k, &mut counter, 0)
    }

    fn backtrack(bytes: &[u8], left: usize, right: usize, k: i32, counter: &mut Vec<i32>, minutes: i32) -> i32 {
        if counter[0] >= k && counter[1] >= k && counter[2] >= k {
            return minutes;
        }
        if left >= right {
            return -1;
        }
        counter[(bytes[left] - b'a') as usize] += 1;
        let left_track = Self::backtrack(bytes, left + 1, right, k, counter, minutes + 1);
        counter[(bytes[left] - b'a') as usize] -= 1;

        counter[(bytes[right] - b'a') as usize] += 1;
        let right_track = Self::backtrack(bytes, left, right - 1, k, counter, minutes + 1);
        counter[(bytes[right] - b'a') as usize] -= 1;

        if left_track != -1 && right_track != -1 {
            left_track.min(right_track)
        } else if left_track != -1 {
            left_track
        } else {
            right_track
        }
    }
}