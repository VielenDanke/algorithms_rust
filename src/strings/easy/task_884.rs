use std::collections::HashSet;


pub struct Solution;

impl Solution {
    pub fn uncommon_from_sentences(s1: String, s2: String) -> Vec<String> {
        let mut words = s1.split(' ').chain(s2.split(' ')).chain(std::iter::once("")).collect::<Vec<_>>();
        words.sort_unstable();
        words.push("");
        words.windows(3)
            .filter_map(|trio| {
                if trio[0] != trio[1] && trio[1] != trio[2] {
                    Some(trio[1])
                } else {
                    None
                }
            })
            .map(String::from)
            .collect()
    }


    pub fn uncommon_from_sentences_map(s1: String, s2: String) -> Vec<String> {
        let (s1, s2) = s1.split_whitespace().into_iter()
            .chain(s2.split_whitespace().into_iter())
            .fold((HashSet::<&str>::new(), HashSet::<&str>::new()),
                  |(mut st1, mut st2), word| {
                      if st1.contains(word) {
                          st2.insert(word);
                      } else {
                          st1.insert(word);
                      }
                      (st1, st2)
                  });
        s1.into_iter().filter(|word| !s2.contains(word)).map(|word| word.to_string()).collect()
    }
}