    pub fn minimum_length(s: String) -> i32 {
        let (mut left, mut right) = (0 as usize, s.len() - 1 as usize);
        let s_bytes = s.as_bytes();

        while left < right {
            let current = s_bytes[left];
            if current == s_bytes[right] {
                while left <= right && s_bytes[left] == current {
                    left += 1;
                }
                while right >= left && s_bytes[right] == current {
                    right -= 1;
                }
            } else {
                break;
            }
        }
        (right - left + 1) as i32
    }
