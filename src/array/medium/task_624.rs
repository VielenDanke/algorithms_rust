pub struct Solution;

impl Solution {
    pub fn max_distance(arrays: Vec<Vec<i32>>) -> i32 {
        let mut global_min = *arrays.first().unwrap().first().unwrap();
        let mut global_max = *arrays.first().unwrap().last().unwrap();
        let mut result = 0;

        for arr in arrays.iter().skip(1) {
            let local_min = *arr.first().unwrap();
            let local_max = *arr.last().unwrap();

            result = result.max((local_max - global_min).abs().max((global_max - local_min).abs()));

            global_min = global_min.min(local_min);
            global_max = global_max.max(local_max);
        }
        result
    }

    pub fn max_distance_brute_force(arrays: Vec<Vec<i32>>) -> i32 {
        let mut arr = vec![vec![1 << 30, -1 << 30]; arrays.len()];

        for (i, current) in arrays.iter().enumerate() {
            for &val in current.iter() {
                arr[i][0] = arr[i][0].min(val);
                arr[i][1] = arr[i][1].max(val);
            }
        }
        let mut result = -1 << 30;

        for i in 0..arrays.len() {
            for j in 0..arrays.len() {
                if i != j {
                    let mut abs = (arr[i][0] - arr[j][1]).abs();
                    abs = abs.max((arr[i][0] - arr[j][0]).abs());
                    abs = abs.max((arr[i][1] - arr[j][0]).abs());
                    abs = abs.max((arr[i][1] - arr[j][1]).abs());
                    result = result.max(abs);
                }
            }
        }
        result
    }
}