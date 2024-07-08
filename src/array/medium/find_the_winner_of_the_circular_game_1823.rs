pub struct Solution;

impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let k = k as usize;
        let n = n as usize;

        let mut arr = vec![0; n];

        for i in 1..=n {
            arr[i - 1] = i;
        }

        let mut next_friend_idx = 0;

        while arr.len() > 1 {
            next_friend_idx += (k - 1);

            if next_friend_idx >= arr.len() {
                next_friend_idx %= arr.len();
            }

            arr = [&arr[..next_friend_idx], &arr[next_friend_idx+1..]].concat();
        }

        arr[0] as i32
    }
}
