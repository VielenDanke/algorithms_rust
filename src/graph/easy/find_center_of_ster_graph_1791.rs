pub struct Solution;

impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        let n = edges.len();

        let mut counter = vec![0; n * 2];

        for coord in edges.iter() {
            let idx_1 = coord[0] as usize;
            let idx_2 = coord[1] as usize;
            counter[idx_1] += 1;
            counter[idx_2] += 1;

            if counter[idx_1] == edges.len() {
                return idx_1 as i32;
            }
            if counter[idx_2] == edges.len() {
                return idx_2 as i32;
            }
        }
        -1
    }
}
