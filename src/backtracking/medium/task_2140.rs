pub struct Solution;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        fn backtrack(questions: &Vec<Vec<i32>>, start: usize, points: i64, dp: &mut Vec<Vec<Option<i64>>>) -> i64 {
            if start >= questions.len() {
                return points;
            }
            let mut max = 0;
            
            for i in start..questions.len() {
                max = max.max(backtrack(questions, i + (questions[i][1] + 1) as usize, points + questions[i][0] as i64, dp));
                max = max.max(backtrack(questions, i + 1, points, dp));
            }
            max
        }
        let mut dp = vec![];
        
        backtrack(&questions, 0, 0, &mut dp)
    }
}