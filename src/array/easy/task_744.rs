pub struct Solution;

impl Solution {
    pub fn next_greatest_letter_sorting(mut letters: Vec<char>, target: char) -> char {
        let to_return = letters[0];

        letters.sort_unstable();

        *letters.iter().find(|&&l| l > target).unwrap_or(&to_return)
    }

    pub fn next_greatest_letter(letters: Vec<char>, target: char) -> char {
        let alph = letters.iter().fold(vec![0; 26], |mut alph, &letter| {
            alph[(letter as u8 - b'a') as usize] += 1;
            alph
        });

        (target..='z')
            .find(|&letter| letter > target && alph[(letter as u8 - b'a') as usize] > 0)
            .unwrap_or(letters[0])
    }
}
