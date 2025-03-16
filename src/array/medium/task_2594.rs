pub struct Solution;

impl Solution {
    pub fn repair_cars(ranks: Vec<i32>, cars: i32) -> i64 {
        let mut left = 0;
        let mut right = (*ranks.iter().min().unwrap() as i64) * (cars as i64) * (cars as i64);
        let mut answer = right;

        while left <= right {
            let mid = (left + right) / 2;
            if Self::can_repair_in_time(mid, &ranks, cars) {
                answer = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        answer
    }

    fn can_repair_in_time(time: i64, ranks: &Vec<i32>, cars: i32) -> bool {
        let mut total_cars = 0;
        
        for &rank in ranks {
            let max_cars = ((time / rank as i64) as f64).sqrt() as i32;
            
            total_cars += max_cars;
            
            if total_cars >= cars {
                return true;
            }
        }
        total_cars >= cars
    }
}