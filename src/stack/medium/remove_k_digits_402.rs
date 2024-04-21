struct Solution {}

impl Solution {
    pub fn remove_kdigits(num: String, k: i32) -> String {
        let mut stack = String::new();
        let mut k = k;

        for current in num.chars() {
            while !stack.is_empty() && k > 0 && stack.chars().last().unwrap() > current {
                stack.remove(stack.len() - 1);
                k -= 1;
            }
            stack.push(current);
        }
        while k > 0 && !stack.is_empty() {
            stack.remove(stack.len() - 1);
            k -= 1;
        }
        stack = stack.trim_start_matches(|c| c == '0').to_string();
        if stack.is_empty() { String::from("0") } else { stack }
    }
}
