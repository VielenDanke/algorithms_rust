struct Solution {}

impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let mut counter = [0; 2];

        for v in students {
            counter[v as usize] += 1;
        }
        for v in sandwiches {
            if counter[v as usize] == 0 {
                return counter[0] + counter[1];
            }
            counter[v as usize] -= 1;
        }
        0
    }
}
