pub struct Solution;

impl Solution {
    pub fn chalk_replacer(chalk: Vec<i32>, mut k: i32) -> i32 {
        let mut current_student = 0;

        loop {
            if current_student >= chalk.len() {
                current_student %= chalk.len();
            }
            let current_chalk = chalk[current_student];

            k -= current_chalk;

            if k < 0 {
                return current_student as i32;
            }
            current_student += 1;
        }
    }

    pub fn chalk_replacer_faster(chalk: Vec<i32>, mut k: i32) -> i32 {
        let total = chalk.iter().map(|&x| x as i64).sum::<i64>();

        let mut remaining = (k as i64 % total) as i32;

        for (i, &current) in chalk.iter().enumerate() {
            if remaining < current {
                return i as i32;
            }
            remaining -= current;
        }
        0
    }
}