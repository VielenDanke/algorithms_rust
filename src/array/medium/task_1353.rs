/*
You are given an array of events where events[i] = [startDayi, endDayi]. Every event i starts at startDayi and ends at endDayi.

You can attend an event i at any day d where startTimei <= d <= endTimei. You can only attend one event at any time d.

Return the maximum number of events you can attend.
 */
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct Solution;

impl Solution {
    pub fn max_events(events: Vec<Vec<i32>>) -> i32 {
        let mut events = events;
        events.sort_by(|a, b| a[0].cmp(&b[0]));
        let max_day = events.iter().map(|e| e[1]).max().unwrap_or(0);
        let mut pq = BinaryHeap::new();
        let mut ans = 0;
        let mut j = 0;
        for i in 1..=max_day {
            while j < events.len() && events[j][0] <= i {
                pq.push(Reverse(events[j][1]));
                j += 1;
            }
            while let Some(&Reverse(end)) = pq.peek() {
                if end < i {
                    pq.pop();
                } else {
                    break;
                }
            }
            if let Some(Reverse(_)) = pq.pop() {
                ans += 1;
            }
        }
        ans
    }
}
