use std::collections::{BTreeSet, HashMap};

pub struct Solution;

impl Solution {

    /// Приватная функция для ребалансировки двух сетов (top_x и rest)
    /// Гарантирует, что:
    /// 1. top_x содержит ровно X элементов (если всего их >= X)
    /// 2. Все элементы в top_x "лучше", чем все элементы в rest
    /// Обновляет sum_top по ходу дела.
    ///
    /// Сложность: O(log U) в среднем (несколько вставок/удалений).
    fn rebalance(
        top_x: &mut BTreeSet<(i32, i32)>,
        rest: &mut BTreeSet<(i32, i32)>,
        x: usize,
        sum_top: &mut i64,
    ) {
        // 1. Перемещаем из rest в top_x, пока в top_x не наберется x
        while top_x.len() < x && !rest.is_empty() {
            // .pop_last() - O(log U) - забирает лучший из 'rest'
            let (best_freq, best_val) = rest.pop_last().unwrap();
            *sum_top += (best_val as i64) * (best_freq as i64);
            top_x.insert((best_freq, best_val)); // O(log U)
        }

        // 2. Перемещаем из top_x в rest, если в top_x их > x
        while top_x.len() > x {
            // .pop_first() - O(log U) - забирает худший из 'top_x'
            let (worst_freq, worst_val) = top_x.pop_first().unwrap();
            *sum_top -= (worst_val as i64) * (worst_freq as i64);
            rest.insert((worst_freq, worst_val)); // O(log U)
        }

        // 3. Меняем местами "худший из лучших" и "лучший из остальных"
        //    (сравниваем top_x.first() и rest.last())
        while let (Some(&(worst_top_freq, worst_top_val)), Some(&(best_rest_freq, best_rest_val))) =
            (top_x.first(), rest.last())
        {
            // Сравнение кортежей (freq, val) напрямую
            if (best_rest_freq, best_rest_val) > (worst_top_freq, worst_top_val) {

                // Удаляем оба элемента
                top_x.pop_first();
                rest.pop_last();

                // Обновляем сумму инкрементально
                *sum_top -= (worst_top_val as i64) * (worst_top_freq as i64);
                *sum_top += (best_rest_val as i64) * (best_rest_freq as i64);

                // Вставляем их крест-накрест
                top_x.insert((best_rest_freq, best_rest_val));
                rest.insert((worst_top_freq, worst_top_val));
            } else {
                // top_x.first() >= rest.last(), значит все в порядке
                break;
            }
        }
    }

    /// Приватная функция для удаления (freq, val) из любого из сетов
    /// (мы не знаем, в каком из них он находится)
    fn remove_from_sets(
        top_x: &mut BTreeSet<(i32, i32)>,
        rest: &mut BTreeSet<(i32, i32)>,
        pair: (i32, i32),
        sum_top: &mut i64,
    ) {
        // .remove() - O(log U)
        if top_x.remove(&pair) {
            // Был в top_x, надо обновить сумму
            *sum_top -= (pair.1 as i64) * (pair.0 as i64);
        } else {
            // Был в rest (или не был, неважно), просто удаляем
            rest.remove(&pair);
        }
    }

    pub fn find_x_sum(nums: Vec<i32>, k: i32, x: i32) -> Vec<i64> {
        let n = nums.len();
        let k_us = k as usize;
        let x_us = x as usize;

        if n < k_us {
            return vec![];
        }
        let mut ans = Vec::with_capacity(n - k_us + 1);

        // Крайний случай
        if x_us == 0 {
            return vec![0i64; n - k_us + 1];
        }

        let mut freq: HashMap<i32, i32> = HashMap::new();
        // top_x хранит X лучших. (first() -> худший из лучших)
        let mut top_x: BTreeSet<(i32, i32)> = BTreeSet::new();
        // rest хранит остальных. (last() -> лучший из остальных)
        let mut rest: BTreeSet<(i32, i32)> = BTreeSet::new();
        let mut sum_top: i64 = 0; // Наш X-Sum

        for i in 0..n {
            let v = nums[i]; // Входящий элемент

            // --- 1. Обработка входящего элемента v ---
            let old_freq_v = freq.get(&v).cloned().unwrap_or(0);
            if old_freq_v > 0 {
                // Удаляем старую запись (old_freq_v, v)
                Self::remove_from_sets(&mut top_x, &mut rest, (old_freq_v, v), &mut sum_top);
            }

            // Обновляем частоту
            let new_freq_v = old_freq_v + 1;
            freq.insert(v, new_freq_v);

            // Добавляем новую запись (new_freq_v, v) в 'rest'
            // 'rest' используется как временная "приемная"
            rest.insert((new_freq_v, v));

            // Ребалансируем (новый элемент может "заслужить" место в top_x)
            Self::rebalance(&mut top_x, &mut rest, x_us, &mut sum_top);

            // --- 2. Обработка исходящего элемента u ---
            if i >= k_us {
                let u = nums[i - k_us]; // Исходящий элемент

                // Получаем его *текущую* частоту (которая > 0)
                let old_freq_u = freq.get(&u).cloned().unwrap();

                // Удаляем текущую запись (old_freq_u, u)
                Self::remove_from_sets(&mut top_x, &mut rest, (old_freq_u, u), &mut sum_top);

                let new_freq_u = old_freq_u - 1;
                if new_freq_u == 0 {
                    // Элемент полностью покинул окно
                    freq.remove(&u);
                } else {
                    // Элемент еще в окне, но с меньшей частотой
                    freq.insert(u, new_freq_u);
                    // Добавляем обновленную запись (new_freq_u, u) в 'rest'
                    rest.insert((new_freq_u, u));
                }

                // Ребалансируем (удаление могло освободить место в top_x)
                Self::rebalance(&mut top_x, &mut rest, x_us, &mut sum_top);
            }

            // --- 3. Запись результата ---
            if i >= k_us - 1 {
                // Окно полностью сформировано
                // sum_top уже содержит правильный x-sum.
                // Если уникальных элементов < x, то 'rest' будет пуст,
                // 'top_x' будет содержать все элементы, и 'sum_top' будет
                // суммой (val*freq) для всех, что совпадает с условием.
                ans.push(sum_top);
            }
        }

        ans
    }
}