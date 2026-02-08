use std::collections::BinaryHeap;

const ALPHABET_SIZE: usize = 26;

#[derive(Clone, Copy)]
struct Conversion {
    target: usize,
    cost: i64,
}

#[derive(Eq, PartialEq)]
struct State {
    cost: i64,
    node: usize,
}

// Реализуем сравнение для State, чтобы BinaryHeap работала как min-heap
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost) // Обратный порядок для стоимости
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub struct Solution;

impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        // 1. Построение графа
        let graph = Self::build_graph(&original, &changed, &cost);

        // 2. Предварительный расчет всех путей (All-Pairs Shortest Path)
        // Используем матрицу 26x26. Option<i64> заменяет -1
        let mut min_costs_matrix = [[None; ALPHABET_SIZE]; ALPHABET_SIZE];
        for i in 0..ALPHABET_SIZE {
            min_costs_matrix[i] = Self::run_dijkstra(i, &graph);
        }

        // 3. Расчет финальной стоимости
        let mut total_cost: i64 = 0;

        // Итерируемся по байтам, так как работаем только с 'a'-'z'
        for (s_byte, t_byte) in source.bytes().zip(target.bytes()) {
            if s_byte == t_byte {
                continue;
            }

            let from = (s_byte - b'a') as usize;
            let to = (t_byte - b'a') as usize;

            match min_costs_matrix[from][to] {
                Some(c) => total_cost += c,
                None => return -1, // Если путь не найден
            }
        }

        total_cost
    }

    fn build_graph(original: &[char], changed: &[char], cost: &[i32]) -> Vec<Vec<Conversion>> {
        let mut graph = vec![vec![]; ALPHABET_SIZE];
        for i in 0..original.len() {
            let from = (original[i] as u8 - b'a') as usize;
            let to = (changed[i] as u8 - b'a') as usize;
            graph[from].push(Conversion {
                target: to,
                cost: cost[i] as i64,
            });
        }
        graph
    }

    fn run_dijkstra(
        start_node: usize,
        graph: &Vec<Vec<Conversion>>,
    ) -> [Option<i64>; ALPHABET_SIZE] {
        let mut dists = [None; ALPHABET_SIZE];
        let mut pq = BinaryHeap::new();

        dists[start_node] = Some(0);
        pq.push(State {
            cost: 0,
            node: start_node,
        });

        while let Some(State { cost, node }) = pq.pop() {
            // Проверка на актуальность пути (уже нашли короче)
            if let Some(best) = dists[node] {
                if cost > best {
                    continue;
                }
            }

            for edge in &graph[node] {
                let new_cost = cost + edge.cost;

                // Если новый путь короче или это первый найденный путь
                if dists[edge.target].map_or(true, |c| new_cost < c) {
                    dists[edge.target] = Some(new_cost);
                    pq.push(State {
                        cost: new_cost,
                        node: edge.target,
                    });
                }
            }
        }
        dists
    }
}
