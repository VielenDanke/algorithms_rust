pub mod easy;
pub mod medium;
pub mod hard;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None
        }
    }
}

pub struct UnionFind {
    pub id: Vec<usize>,
    pub rank: Vec<usize>,
    pub count: i32,
}

impl UnionFind {
    pub fn build(n: usize) -> UnionFind {
        let mut rank = vec![0; n * n];
        let mut parent = vec![0; n * n];
        let mut count = 0;

        for i in 0..parent.len() {
            parent[i] = i;
            rank[i] = 1;
        }

        UnionFind {
            id: parent,
            rank,
            count,
        }
    }

    pub fn union(&mut self, x: usize, y: usize) {
        let parent_x = self.find(x);
        let parent_y = self.find(y);
        if parent_x == parent_y {
            self.count += 1;
        } else {
            if self.rank[parent_x] > self.rank[parent_y] {
                self.id[parent_y] = parent_x;
            } else if self.rank[parent_x] < self.rank[parent_y] {
                self.id[parent_x] = parent_y;
            } else {
                self.id[parent_y] = parent_x;
                self.rank[parent_x] += 1;
            }
        }
    }

    pub fn find(&mut self, mut p: usize) -> usize {
        while p != self.id[p] {
            self.id[p] = self.id[self.id[p]];
            p = self.id[p];
        }
        p
    }
}