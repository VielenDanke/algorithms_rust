pub struct Solution;

impl Solution {
    pub fn rotate_string(s: String, goal: String) -> bool {
        if s.len() != goal.len() {
            false
        } else {
            (s.clone() + &s).contains(&goal)
        }
    }

    pub fn rotate_string_brute_force(s: String, goal: String) -> bool {
        let mut s_bytes = s.as_bytes().to_vec();
        let goal_bytes = goal.as_bytes();

        let n = s_bytes.len();

        if n != goal_bytes.len() {
            return false;
        }

        for i in 0..n {
            let mut is_found = true;
            for j in 0..n {
                if s_bytes[j] != goal_bytes[j] {
                    is_found = false;
                    break;
                }
            }
            if is_found {
                return true;
            }
            let removed = s_bytes.remove(0);
            s_bytes.push(removed);
        }
        false
    }
}