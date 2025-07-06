pub struct Solution;

impl Solution {
    pub fn find_lucky(arr: Vec<i32>) -> i32 {
        let mut counter = vec![0; 501];

        for &v in arr.iter() {
            counter[v as usize] += 1;
        }
        let mut largest = -1;

        for &v in arr.iter() {
            if v == counter[v as usize] && largest < v {
                largest = v;
            }
        }
        largest
    }
}