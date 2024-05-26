pub struct Solution;
const BIG_MOD: u32 = 1000000007;

impl Solution {

    pub fn check_record(n: i32) -> i32 {
        let mut absences_lates: [[u32; 3]; 2] = [[1, 0, 0], [0, 0, 0]];
        let mut new_absences_lates = [[0; 3]; 2];

        for _ in 0..n {
            // Mark "P", erases consecutive "L"s and does not change "A"s
            new_absences_lates[0][0] = (absences_lates[0][0] + absences_lates[0][1] + absences_lates[0][2]) % BIG_MOD;
            new_absences_lates[1][0] = (absences_lates[1][0] + absences_lates[1][1] + absences_lates[1][2]) % BIG_MOD;

            // Mark "A", increases "A"s from 0 to 1 but erases consecutive "L"s
            new_absences_lates[1][0] = (new_absences_lates[1][0] + new_absences_lates[0][0]) % BIG_MOD;

            // Marking "L", increases "L"s from 0 to 1 or 1 to 2, does not change "A"s
            // Only way to get to 1 or 2 lates is by marking L
            new_absences_lates[0][1] = absences_lates[0][0];
            new_absences_lates[0][2] = absences_lates[0][1];
            new_absences_lates[1][1] = absences_lates[1][0];
            new_absences_lates[1][2] = absences_lates[1][1];

            absences_lates = new_absences_lates;
        }
        (absences_lates
            .into_iter()
            .map(|lates| lates.into_iter().sum::<u32>() % BIG_MOD)
            .sum::<u32>() % BIG_MOD) as i32
    }

    pub fn check_record_backtracking_memo(n: i32) -> i32 {
        let mut cache = vec![vec![vec![-1i64; 5]; 5]; 100005];
        return (Self::form_possible_attendance(&mut cache, n, 0, 0, &vec!['A', 'L', 'P']) % BIG_MOD) as i32;
    }

    fn form_possible_attendance(cache: &mut Vec<Vec<Vec<i64>>>, n: i32, absence: i32, consecutive: i32, ch_arr: &Vec<char>) -> i64 {
        if n == 0 {
            return 1;
        }
        if cache[n as usize][absence as usize][consecutive as usize] > -1 {
            return cache[n as usize][absence as usize][consecutive as usize];
        }
        let mut result = 0;
        for ch in ch_arr {
            let new_absence = if ch == &ch_arr[0] { absence + 1 } else { absence };
            if new_absence >= 2 {
                continue;
            }
            let new_consecutive = if ch == &ch_arr[1] { consecutive + 1 } else { 0 };
            if new_consecutive >= 3 {
                continue;
            }
            result += Self::form_possible_attendance(cache, n - 1, new_absence, new_consecutive, ch_arr);
            result %= BIG_MOD;
        }
        cache[n as usize][absence as usize][consecutive as usize] = result;
        result
    }
}
