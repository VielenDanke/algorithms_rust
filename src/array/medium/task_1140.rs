pub struct Solution;

impl Solution {
    pub fn stone_game_ii(mut piles: Vec<i32>) -> i32 {
        // 1 <= X <= 2M
        // M = 1 Alice starts
        // M = max(M, X)
        let n = piles.len();

        for i in (0..(n - 1)).rev() {
            piles[i] += piles[i + 1];
        }
        fn recursive(psum: &[i32], m: usize, memory: &mut Vec<Vec<i32>>) -> i32 {
            let n = psum.len();
            if n <= 2 * m {
                return if n == 0 { 0 } else { psum[0] };
            }
            if memory[n - 1][m] != -1 {
                return memory[n - 1][m];
            }
            let min = (1..=(2 * m).min(psum.len()))
                .map(|k| recursive(&psum[k..], m.max(k), memory))
                .min()
                .unwrap();
            memory[n-1][m] = psum[0] - min;
            psum[0] - min
        }
        recursive(&piles, 1, &mut vec![vec![-1; 33]; piles.len()])
    }
}

