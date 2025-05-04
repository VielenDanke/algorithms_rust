use std::collections::HashMap;
use std::hash::{Hash, Hasher};

pub struct Solution;

#[derive(Debug, Eq)]
pub struct Pair {
    pub first: i32,
    pub second: i32,
}

impl PartialEq for Pair {
    fn eq(&self, other: &Self) -> bool {
        (self.first == other.first && self.second == other.second)
            || (self.first == other.second && self.second == other.first)
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl Hash for Pair {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Sort the pair so the hash is independent of order
        let (min, max) = if self.first < self.second {
            (self.first, self.second)
        } else {
            (self.second, self.first)
        };
        min.hash(state);
        max.hash(state);
    }
}

impl Solution {
    pub fn num_equiv_domino_pairs_brute_force(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut counter = 0;
        let n = dominoes.len();
        for i in 0..n {
            for j in i + 1..n {
                let i_domino = &dominoes[i];
                let j_domino = &dominoes[j];
                if (i_domino[0] == j_domino[0] && i_domino[1] == j_domino[1])
                    || (i_domino[0] == j_domino[1] && i_domino[1] == j_domino[0])
                {
                    counter += 1;
                }
            }
        }
        counter
    }

    pub fn num_equiv_domino_pairs_brute(dominoes: Vec<Vec<i32>>) -> i32 {
        let mut m: HashMap<Pair, i32> = HashMap::new();
        let mut count = 0;

        for d in dominoes {
            let pair = Pair {
                first: d[0],
                second: d[1],
            };
            if let Some(&existing) = m.get(&pair) {
                count += existing;
            }
            *m.entry(pair).or_insert(0) += 1;
        }

        count
    }
}
