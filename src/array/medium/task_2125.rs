pub struct Solution;

impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        bank.iter()
            .map(|row| row.as_bytes().iter().filter(|&&v| v == b'1').count() as i32)
            .filter(|&v| v != 0)
            .fold((0, 0), |(result, prev), current| (result + prev * current, current))
            .0
    }
}
