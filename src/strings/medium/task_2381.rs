pub struct Solution;

impl Solution {
    pub fn shifting_letters(s: String, shifts: Vec<Vec<i32>>) -> String {
        let n = s.len();
        let mut shift_arr = vec![0; n];

        for shift in shifts {
            let start = shift[0] as usize;
            let end = shift[1] as usize;
            let direction = shift[2];
            
            shift_arr[start] += if direction == 1 { 1 } else { -1 };
            
            if end + 1 < n {
                shift_arr[end + 1] -= if direction == 1 { 1 } else { -1 };
            }
        }

        for i in 1..n {
            shift_arr[i] += shift_arr[i - 1];
        }

        let mut result = String::new();
        
        for (i, char) in s.chars().enumerate() {
            let mut new_code = char as i32 - 'a' as i32 + shift_arr[i];
            new_code = (new_code % 26 + 26) % 26;
            result.push((new_code + 'a' as i32) as u8 as char);
        }

        result
    }
}
