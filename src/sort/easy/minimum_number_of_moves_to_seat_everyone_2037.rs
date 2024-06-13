pub struct Solution;

impl Solution {
    pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
        seats.sort_unstable();
        students.sort_unstable();

        let mut result = 0;

        for i in 0..seats.len() {
            result += seats[i].abs_diff(students[i])
        }
        
        result as i32
    }

    pub fn min_moves_to_seat_count_sort(seats: Vec<i32>, students: Vec<i32>) -> i32 {
        let mut counter = vec![0i32; 101];

        for i in 0..seats.len() {
            counter[seats[i] as usize] += 1;
            counter[students[i] as usize] -= 1;
        }

        let (mut result, mut current) = (0i32, 0i32);

        for &count in counter.iter() {
            result += current.abs();
            current += count;
        }

        result
    }
}
