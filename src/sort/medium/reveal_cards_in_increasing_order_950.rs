pub struct Solution {}

impl Solution {
    pub fn deck_revealed_increasing(deck: Vec<i32>) -> Vec<i32> {
        let mut deck = Vec::from(deck);
        let mut skip = false;
        let mut idx_deck = 0;
        let mut idx_res = 0;
        let n = deck.len();
        let mut result = vec![0i32; n];

        deck.sort();

        while idx_deck < n {
            if result[idx_res] == 0 {
                if !skip {
                    result[idx_res] = deck[idx_deck];
                    idx_deck += 1;
                }
                skip = !skip;
            }
            idx_res = (idx_res + 1) % n;
        }
        result
    }

    pub fn deck_revealed_increasing_queue(deck: Vec<i32>) -> Vec<i32> {
        let n = deck.len();
        let mut deck = Vec::from(deck);
        let mut result = vec![0i32; n];
        let mut queue = Vec::new();

        for i in 0..n {
            queue.push(i);
        }
        deck.sort();

        for i in 0..n {
            let idx = queue.remove(0);
            result[idx] = deck[i];
            if !queue.is_empty() {
                let to_bottom = queue.remove(0);
                queue.push(to_bottom);
            }
        }
        result
    }
}
