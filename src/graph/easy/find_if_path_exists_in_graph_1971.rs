use std::cell::RefCell;

struct Solution {}

struct UnionFind {
    id: RefCell<Vec<i32>>
}

impl UnionFind {

    fn new(n: i32) -> UnionFind {
        UnionFind {
            id: RefCell::new((0..n).collect()),
        }
    }

    fn find(&self, mut p: i32) -> i32 {
        let mut id_mut = self.id.borrow_mut();
        while p != id_mut[p as usize] {
            id_mut[p as usize] = id_mut[id_mut[p as usize] as usize];
            p = id_mut[p as usize];
        }
        p
    }

    fn union(&self, p: i32, q: i32) {
        let p_find = self.find(p);
        let q_find = self.find(q);
        let mut id_mut = self.id.borrow_mut();
        id_mut[p_find as usize] = q_find;
    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let uf = UnionFind::new(n);

        edges.iter().for_each(|edge| uf.union(edge[0], edge[1]));

        uf.find(source) == uf.find(destination)
    }
}
