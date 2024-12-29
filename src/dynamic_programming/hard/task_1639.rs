pub struct Solution;

impl Solution {
    pub fn num_ways(words: Vec<String>, target: String) -> i32 {
        fn get_words(
            words: &Vec<String>,
            target: &String,
            words_idx: usize,
            target_idx: usize,
            dp: &mut Vec<Vec<Option<i64>>>,
            char_frequency: &mut Vec<Vec<i64>>,
        ) -> i64 {
            if target_idx == target.len() {
                return 1;
            }
            if words_idx == words[0].len() || words[0].len() - words_idx < target.len() - target_idx
            {
                return 0;
            }
            if dp[words_idx][target_idx].is_some() {
                return dp[words_idx][target_idx].unwrap();
            }
            let mut count_ways = 0i64;
            let mut current_position = (target.as_bytes()[target_idx] - b'a') as usize;

            count_ways += get_words(words, target, words_idx + 1, target_idx, dp, char_frequency);
            count_ways += char_frequency[words_idx][current_position]
                * get_words(
                    words,
                    target,
                    words_idx + 1,
                    target_idx + 1,
                    dp,
                    char_frequency,
                );

            dp[words_idx][target_idx] = Some(count_ways % 1000000007);

            dp[words_idx][target_idx].unwrap()
        }
        let word_len = words[0].len();
        let target_len = target.len();

        let mut dp = vec![vec![None; target_len]; word_len];

        let mut char_frequency = vec![vec![0i64; 26]; word_len];

        for word in words.iter() {
            for (j, &ch) in word.as_bytes().iter().enumerate() {
                char_frequency[j][(ch - b'a') as usize] += 1;
            }
        }
        get_words(&words, &target, 0, 0, &mut dp, &mut char_frequency) as i32
    }
}
