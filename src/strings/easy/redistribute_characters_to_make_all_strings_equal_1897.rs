pub fn make_equal(words: Vec<String>) -> bool {
    let mut alph = vec![0; 26];

    for word in &words {
        for c in word.chars() {
            alph[(c as u8 - b'a') as usize] += 1;
        }
    }
    alph.iter().all(|&count| count % words.len() == 0)
}
