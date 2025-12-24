pub struct Solution;

impl Solution {
    pub fn minimum_boxes(apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        let mut total_apples: i32 = apple.iter().sum();

        capacity.sort_unstable_by(|a, b| b.cmp(a));

        let mut boxes_used = 0;

        for box_size in capacity {
            if total_apples <= 0 {
                break;
            }
            total_apples -= box_size;
            boxes_used += 1;
        }

        boxes_used
    }

    pub fn minimum_boxes_brute_force(mut apple: Vec<i32>, mut capacity: Vec<i32>) -> i32 {
        apple.sort_unstable();
        capacity.sort_unstable();

        let n = apple.len() as i32;
        let mut i = n - 1;
        let m = capacity.len() as i32;
        let mut j = m - 1;

        while i >= 0 && j >= 0 {
            let a_i = apple[i as usize];
            let c_j = capacity[j as usize];

            if c_j - a_i > 0 {
                capacity[j as usize] -= a_i;
                apple[i as usize] = 0;
                i -= 1;
                if i < 0 {
                    return m - j;
                }
            } else if c_j - a_i < 0 {
                capacity[j as usize] = 0;
                apple[i as usize] = a_i - c_j;
                j -= 1;
            } else {
                capacity[j as usize] = 0;
                apple[i as usize] = 0;
                i -= 1;
                if i < 0 {
                    return m - j;
                }
                j -= 1;
            }
        }

        m - j
    }
}