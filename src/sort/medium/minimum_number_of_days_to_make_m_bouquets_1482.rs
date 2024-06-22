pub struct Solution;

impl Solution {
    pub fn min_days(mut bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        // m buckets
        // k adjacent flowers
        let (mut start, mut end) = (0, 0);

        for &day in bloom_day.iter() {
            end = end.max(day);
        }

        let mut min_days = -1;

        while start <= end {
            let mid = start + (end - start) / 2;

            if Self::get_number_of_bouquets(&bloom_day, mid, k) >= m {
                min_days = mid;
                end = mid - 1;
            } else {
                start = mid + 1;
            }
        }

        min_days
    }

    fn get_number_of_bouquets(bloom_day: &Vec<i32>, mid: i32, k: i32) -> i32 {
        let (mut num_of_bouquets, mut count) = (0, 0);

        for &day in bloom_day.iter() {
            if day <= mid {
                count += 1;
            } else {
                count = 0;
            }

            if count == k {
                num_of_bouquets += 1;
                count = 0;
            }
        }

        num_of_bouquets
    }

    pub fn min_days_tle(mut bloom_day: Vec<i32>, m: i32, k: i32) -> i32 {
        // m buckets
        // k adjacent flowers
        if bloom_day.len() < (m * k) as usize {
            return -1;
        }

        Self::backtrack(bloom_day, &mut Vec::new(), m, k as usize)
    }

    fn backtrack(bloom_day: Vec<i32>, current_blooms: &mut Vec<i32>, m: i32, k: usize) -> i32 {
        if m == 0 {
            return *current_blooms.iter().max().unwrap();
        }

        if bloom_day.is_empty() {
            return 1 << 30;
        }

        let mut idx = 0usize;

        let mut current_min = 1 << 30;

        while idx + k <= bloom_day.len() {
            current_blooms.push(*bloom_day[idx..idx + k].iter().max().unwrap());

            current_min = current_min.min(Self::backtrack(bloom_day[idx + k..].to_vec(), current_blooms, m - 1, k));

            current_blooms.remove(current_blooms.len() - 1);

            idx += 1;
        }

        current_min
    }
}
