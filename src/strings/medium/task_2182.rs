pub struct Solution;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        // create alph array
        // count all letters
        // go from the end
        // take limit than move forward, if repeat_limit < letters[i], take one letter from next
        // and repeat from current
        let mut alph = vec![0; 26];
        let mut result_bytes = Vec::new();
        let mut last_index = None;
        let mut idx = 25;

        for &ch in s.as_bytes() {
            alph[(ch - b'a') as usize] += 1;
        }
        loop {
            if alph[idx] > 0 {
                if last_index.is_none() {
                    let current_letter = alph[idx];
                    for _ in 0..current_letter.min(repeat_limit) {
                        result_bytes.push(idx as u8 + b'a');
                        alph[idx] -= 1;
                    }
                    if alph[idx] > 0 {
                        last_index = Some(idx);
                    }
                } else {
                    result_bytes.push(idx as u8 + b'a');
                    alph[idx] -= 1;
                    idx = last_index.take().unwrap();
                    continue;
                }
            }
            if idx == 0 {
                break;
            }
            idx -= 1;
        }

        String::from_utf8(result_bytes).unwrap()
    }
}
