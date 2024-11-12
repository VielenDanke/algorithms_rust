pub struct Solution;

impl Solution {
    pub fn maximum_beauty(mut items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        items.sort_unstable_by_key(|k| (-k[1], k[0]));
        items.dedup_by(|a, b| a[0] >= b[0]);
        queries
            .into_iter()
            .map(|q| match items.binary_search_by_key(&-q, |k| -k[0]) {
                Ok(i) | Err(i) => items.get(i).map_or(0, |k| k[1]),
            })
            .collect()
    }

    pub fn maximum_beauty_brute_force(items: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let (n, m) = (items.len(), queries.len());
        let mut result = Vec::with_capacity(n);
        for i in 0..m {
            let query = queries[i];
            let mut current_max = 0;
            for j in 0..n {
                if items[j][0] <= query {
                    current_max = current_max.max(items[j][1]);
                }
            }
            result.push(current_max);
        }
        result
    }
}