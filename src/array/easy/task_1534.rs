pub struct Solution;

impl Solution {
    pub fn count_good_triplets(arr: Vec<i32>, a: i32, b: i32, c: i32) -> i32 {
        let m = arr.len();
        let mut result = 0;

        for i in 0..m {
            for j in i + 1..m {
                for k in j + 1..m {
                    if (arr[i] - arr[j]).abs() <= a && (arr[j] - arr[k]).abs() <= b && (arr[i] - arr[k]).abs() <= c {
                        result += 1;
                    }
                }
            }
        }
        result
    }
}