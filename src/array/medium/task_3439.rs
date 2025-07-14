/*
You are given an integer eventTime denoting the duration of an event, where the event occurs from time t = 0 to time t = eventTime.

You are also given two integer arrays startTime and endTime, each of length n. These represent the start and end time of n non-overlapping meetings, where the ith meeting occurs during the time [startTime[i], endTime[i]].

You can reschedule at most k meetings by moving their start time while maintaining the same duration, to maximize the longest continuous period of free time during the event.

The relative order of all the meetings should stay the same and they should remain non-overlapping.

Return the maximum amount of free time possible after rearranging the meetings.

Note that the meetings can not be rescheduled to a time outside the event.
 */

pub struct Solution;

impl Solution {
    pub fn max_free_time(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        // track free spots
        // if there is on right side which fits free spot - move it
        // do the same from other side
        let n = start_time.len();
        let k = k as usize;
        let mut res: i32 = 0;
        let mut t: i32 = 0;

        for i in 0..n {
            t += end_time[i] - start_time[i];
            let left = if i <= k - 1 { 0 } else { end_time[i - k] };
            let right = if i == n - 1 {
                event_time
            } else {
                start_time[i + 1]
            };
            res = res.max(right - left - t);
            if i >= k - 1 {
                t -= end_time[i - k + 1] - start_time[i - k + 1];
            }
        }
        res
    }

    pub fn max_free_time_slower(event_time: i32, k: i32, start_time: Vec<i32>, end_time: Vec<i32>) -> i32 {
        // 1. find free time amount between meetings
        // (including zero free time and before the first meeting and after the last meeting)
        let mut free_times: Vec<i32> = vec![];
        let mut last_end = 0;

        for (start, end) in start_time.iter().zip(end_time.iter()) {
            let free_time = start - last_end;
            free_times.push(free_time);
            last_end = *end;
        }
        // free time after the last meeting
        free_times.push(event_time - last_end);

        // 2. find max consecutive sum of the free times using sliding window size k+1
        // because if we can move k meeting, that means k+1 consequent free times we can have.
        let window_size = (k + 1) as usize;
        let mut max_free_time = i32::MIN;
        for i in 0..free_times.len() - window_size + 1 {
            let mut sum = 0;
            for j in 0..window_size {
                sum += free_times[i + j];
            }
            if sum > max_free_time {
                max_free_time = sum;
            }
        }

        max_free_time
    }
}

