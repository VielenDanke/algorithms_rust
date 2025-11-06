use std::collections::{BTreeSet, HashMap};

pub struct Solution;

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, mut i: usize) -> usize {
        while i != self.parent[i] {
            self.parent[i] = self.parent[self.parent[i]];
            i = self.parent[i];
        }
        i
    }

    pub fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            return false;
        }

        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }

        self.count -= 1;
        true
    }

    pub fn is_connected(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }

    pub fn set_count(&self) -> usize {
        self.count
    }
}

impl Solution {
    // queries[0] == 2 - уходит оффлайн
    // queries[0] == 1 - проверить станцию
    // если оффлайн - вернуть наименьший id в той же сети
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = (c + 1) as usize;
        let mut uf = UnionFind::new(n);

        for conn in connections {
            uf.union(conn[0] as usize, conn[1] as usize);
        }

        let mut station_to_root = vec![0; n];
        let mut components: HashMap<usize, BTreeSet<usize>> = HashMap::new();

        for i in 1..n {
            let root = uf.find(i);
            station_to_root[i] = root;
            components.entry(root).or_default().insert(i);
        }

        let mut result = vec![];

        for query in queries {
            let station_id = query[1] as usize;

            if query[0] == 1 {
                let root = station_to_root[station_id];

                if let Some(members) = components.get(&root) {
                    if members.contains(&station_id) {
                        result.push(station_id as i32);
                    } else {
                        if let Some(&smallest_online) = members.iter().next() {
                            result.push(smallest_online as i32);
                        } else {
                            result.push(-1);
                        }
                    }
                } else {
                    result.push(-1);
                }
            } else {
                let root = station_to_root[station_id];

                if let Some(members) = components.get_mut(&root) {
                    members.remove(&station_id); // O(log k)
                }
            }
        }
        result
    }
}
