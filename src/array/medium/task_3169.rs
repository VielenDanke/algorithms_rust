use std::collections::LinkedList;

pub struct Solution;

impl Solution {
    pub fn count_days_line_sweep(mut days: i32, meetings: Vec<Vec<i32>>) -> i32 {
        let mut sweep = vec![0; (days + 1) as usize];
        
        for meeting in meetings {
            for i in meeting[0]..meeting[1] + 1 {
                let i = i as usize;
                if sweep[i] == 0 {
                    sweep[i] += 1;
                    days -= 1;
                }
            }
        }
        sweep.into_iter().filter(|&x| x == 0).count() as i32 - 1
    }
    
    pub fn count_days_brute_force_sort(days: i32, mut meetings: Vec<Vec<i32>>) -> i32 {
        meetings.sort_unstable_by_key(|m| m[0]);
        
        let (mut free_days, mut latest_end) = (0, 0);
        
        for meeting in meetings {
            let (start, end) = (meeting[0], meeting[1]);
            
            if start > latest_end + 1 {
                free_days += start - latest_end - 1;
            }
            latest_end = latest_end.max(end);
        }
        
        free_days + days - latest_end
    }
}
