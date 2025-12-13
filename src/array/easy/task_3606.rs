use itertools::Itertools;

pub struct Solution;

impl Solution {
    fn validate_coupons(codes: Vec<String>, business_lines: Vec<String>, is_active: Vec<bool>) -> Vec<String> {
        (0..codes.len())
            .filter(|&i| is_active[i])
            .filter(|&i| Self::is_code_valid(&codes[i]))
            .filter_map(|i| Self::get_business_priority(&business_lines[i], &codes[i]))
            .sorted_unstable()
            .map(|(_, code)| code)
            .collect()
    }

    fn get_business_priority(business_line: &str, code: &String) -> Option<(u8, String)> {
        match business_line {
            "electronics" => Some((0, code.clone())),
            "grocery" => Some((1, code.clone())),
            "pharmacy" => Some((2, code.clone())),
            "restaurant" => Some((3, code.clone())),
            _ => None,
        }
    }

    fn is_code_valid(code: &str) -> bool {
        if code.is_empty() {
            return false;
        }
        for ch in code.chars() {
            if !ch.is_alphanumeric() && ch != '_' {
                return false;
            }
        }
        true
    }
}