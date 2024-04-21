pub fn first_uniq_char(s: String) -> i32 {
    let mut alph: [i32; 26] = [0; 26];

    for ch in s.chars() {
        alph[ch as usize - 'a' as usize] += 1;
    }
    for (i, ch) in s.chars().enumerate() {
        if alph[ch as usize - 'a' as usize] == 1 {
            return i as i32;
        }
    }
    -1
}
