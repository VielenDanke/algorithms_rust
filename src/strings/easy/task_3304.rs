/*
Alice and Bob are playing a game. Initially, Alice has a string word = "a".

You are given a positive integer k.

Now Bob will ask Alice to perform the following operation forever:

Generate a new string by changing each character in word to its next character in the English alphabet, and append it to the original word.
For example, performing the operation on "c" generates "cd" and performing the operation on "zb" generates "zbac".

Return the value of the kth character in word, after enough operations have been done for word to have at least k characters.

Note that the character 'z' can be changed to 'a' in the operation.
 */

pub struct Solution;

impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut original = vec![0];
        let mut idx = 1;
        let k_usize = k as usize;

        while original.len() < k_usize {
            for i in 0..idx {
                original.push((original[i] + 1) % 27);
            }
            idx = original.len();
        }
        (original[k_usize - 1] + b'a') as char
    }
}
