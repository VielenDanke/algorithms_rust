pub struct Solution;

impl Solution {
    pub fn xor_queries(mut arr: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut sum = 0;
        arr.insert(0, 0);
        let prefix = arr.into_iter()
            .map(|val| {
                sum ^= val;
                sum
            })
            .collect::<Vec<i32>>();
        queries.into_iter()
            .map(|query| prefix[query[0] as usize] ^ prefix[(query[1] + 1) as usize])
            .collect()
    }
}