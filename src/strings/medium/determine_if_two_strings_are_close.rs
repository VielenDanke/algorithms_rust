pub fn close_strings(word1: String, word2: String) -> bool {
    let mut count1 = vec![0; 26];
    let mut count2 = vec![0; 26];

    for ch in word1.chars() {
        count1[ch as usize - 'a' as usize] += 1
    }
    for ch in word2.chars() {
        count2[ch as usize - 'a' as usize] += 1
    }
    for i in 0..26 {
        if (count1[i] > 0 && count2[i] == 0) || (count1[i] == 0 && count2[i] > 0) {
            return false;
        }
    }
    count1.sort();
    count2.sort();

    for i in 0..26 {
        if count1[i] != count2[i] {
            return false;
        }
    }
    true
}
