use std::collections::BTreeSet;

pub struct Solution;

impl Solution {
    pub fn find_lex_smallest_string(s: String, a: i32, b: i32) -> String {
        // odd indices only
        // b move to the right
        // a to add
        // track what is on the first

        fn rotate_vec_right(arr: &mut Vec<u8>, to_rotate: usize) {
            let len = arr.len();

            if len == 0 {
                return;
            }

            let actual_b = to_rotate % len;

            arr.rotate_right(actual_b);
        }

        fn add_all_odd_indices(arr: &mut Vec<u8>, to_add: u8) {
            const ZERO_ASCII: u8 = b'0';

            for i in 0..arr.len() {
                if i % 2 != 0 {
                    // Нечетные индексы (1, 3, 5, ...)
                    let digit_value = arr[i] - ZERO_ASCII;

                    let new_digit = (digit_value + to_add) % 10;

                    arr[i] = new_digit + ZERO_ASCII;
                }
            }
        }

        fn dfs(s: &Vec<u8>, a: i32, b: i32, visited: &mut BTreeSet<String>) {
            let current_string = String::from_utf8(s.clone()).unwrap();

            if !visited.insert(current_string) {
                return;
            }

            let mut cloned_arr_rotate = s.clone();
            rotate_vec_right(&mut cloned_arr_rotate, b as usize);
            dfs(&cloned_arr_rotate, a, b, visited);

            let mut cloned_arr_add = s.clone();
            add_all_odd_indices(&mut cloned_arr_add, a as u8);
            dfs(&cloned_arr_add, a, b, visited);
        }

        let mut visited = BTreeSet::new();

        dfs(&s.as_bytes().to_vec(), a, b, &mut visited);

        visited.pop_first().unwrap().to_string()
    }

    pub fn find_lex_smallest_string_gcd(s: String, a: i32, b: i32) -> String {
        fn gcd(mut a: usize, mut b: usize) -> usize {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        }

        let n = s.len();
        let mut res = s.clone();
        let doubled = s.repeat(2);
        let g = gcd(b as usize, n);

        let add = |t: &mut Vec<char>, start: usize| {
            let original = t[start].to_digit(10).unwrap() as i32;
            let (mut min_val, mut times) = (10, 0);
            for i in 0..10 {
                let added = (original + i * a) % 10;
                if added < min_val {
                    min_val = added;
                    times = i;
                }
            }
            for i in (start..n).step_by(2) {
                let digit = t[i].to_digit(10).unwrap() as i32;
                t[i] = std::char::from_digit(((digit + times * a) % 10) as u32, 10).unwrap();
            }
        };

        for i in (0..n).step_by(g) {
            let mut t: Vec<char> = doubled[i..i+n].chars().collect();
            add(&mut t, 1);
            if b % 2 != 0 {
                add(&mut t, 0);
            }
            let t_str: String = t.into_iter().collect();
            if t_str < res {
                res = t_str;
            }
        }
        res
    }
}
