use std::cell::RefCell;

struct Solution {}

struct UnionFind {
    id: RefCell<Vec<usize>>,
    sizes: RefCell<Vec<usize>>,
    num_of_components: RefCell<usize>,
}

impl UnionFind {

    fn new(n: usize) -> UnionFind {
        UnionFind {
            id: RefCell::new((0..n).collect()),
            sizes: RefCell::new((0..n).collect()),
            num_of_components: RefCell::new(n),
        }
    }

    fn find(&self, mut p: usize) -> usize {
        let mut id_mut = self.id.borrow_mut();
        while p != id_mut[p] {
            id_mut[p] = id_mut[id_mut[p]];
            p = id_mut[p];
        }
        p
    }

    fn union(&self, p: usize, q: usize) {
        let p_find = self.find(p);
        let q_find = self.find(q);
        if q_find == p_find {
            return;
        }
        let mut sizes = self.sizes.borrow_mut();
        let mut id = self.id.borrow_mut();
        if sizes[p_find] < sizes[q_find] {
            sizes[q_find] += sizes[p_find];
            id[p_find] = q_find;
        } else {
            sizes[p_find] += sizes[q_find];
            id[q_find] = p_find;
        }
        *self.num_of_components.borrow_mut() -= 1;
    }
}

impl Solution {
    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let uf = UnionFind::new(n as usize);

        edges.iter().for_each(|edge| uf.union(edge[0] as usize, edge[1] as usize));

        uf.find(source as usize) == uf.find(destination as usize)
    }
}
