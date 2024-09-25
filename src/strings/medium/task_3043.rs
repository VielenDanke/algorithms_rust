pub struct Solution;

use std::collections::{HashMap, HashSet};

#[derive(Default, Debug)]
struct TrieNode {
    children: HashMap<char, TrieNode>,
}

#[derive(Default, Debug)]
pub struct Trie {
    root: TrieNode,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            root: TrieNode::default(),
        }
    }

    pub fn insert(&mut self, num: &i32) {
        let mut node = &mut self.root;
        let num_str = num.to_string();
        for digit in num_str.chars() {
            node = node.children.entry(digit).or_default();
        }
    }

    pub fn find_longest_prefix(&self, num: &i32) -> i32 {
        let mut node = &self.root;
        let num_str = num.to_string();
        let mut len = 0;
        for digit in num_str.chars() {
            match node.children.get(&digit) {
                Some(node2) => {
                    node = node2;
                    len += 1;
                },
                None => break,
            }
        }
        len
    }
}

impl Solution {
    pub fn longest_common_prefix_tle_strings(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let s1 = arr1.iter().map(i32::to_string).collect::<Vec<String>>();
        let s2 = arr2.iter().map(i32::to_string).collect::<Vec<String>>();

        let mut max = 0;

        for first in s1.iter() {
            for second in s2.iter() {
                let mut counter = 0;
                for i in 0..first.len().min(second.len()) {
                    if first.as_bytes()[i] == second.as_bytes()[i] {
                        counter += 1;
                    } else {
                        break;
                    }
                }
                max = max.max(counter);
            }
        }
        max
    }

    pub fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut trie = Trie::new();
        for num in arr1 {
            trie.insert(&num);
        }
        let mut ans = 0;
        for num in arr2 {
            ans = ans.max(trie.find_longest_prefix(&num))
        }
        ans
    }

    pub fn longest_common_prefix_i32(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut data = HashSet::new();

        for mut v in arr1 {
            while v != 0 {
                data.insert(v);
                v /= 10;
            }
        }
        let mut max = -1;

        for mut v in arr2 {
            while v != 0 {
                if data.contains(&v) {
                    max = max.max(v);
                    break;
                }
                v /= 10;
            }
        }
        if max == -1 {
            return 0;
        }
        let mut counter = 0;

        while max != 0 {
            max /= 10;
            counter += 1;
        }
        counter
    }
}