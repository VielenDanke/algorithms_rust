use std::collections::{BTreeSet, HashMap};

pub struct Solution;

impl Solution {
    /// Решает задачу "Avoid Flood in The City".
    ///
    /// # Аргументы
    /// * `rains`: Вектор, где `rains[i]` — это номер озера, в которое пошел дождь
    ///            в день `i`, или `0`, если это сухой день.
    ///
    /// # Возвращает
    /// Вектор `ans`, где `ans[i]` — это действие в день `i`:
    /// * `ans[i] = -1`: Если в день `i` идет дождь (заполняется в коде).
    /// * `ans[i] = k > 0`: Если в сухой день `i` мы осушаем озеро `k`.
    /// * `ans[i] = 1`: Если в сухой день `i` мы ничего не делаем (заполняется по умолчанию).
    ///
    /// Возвращает пустой вектор (`Vec::new()`) в случае невозможности избежать наводнения.
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        let n = rains.len();

        // 1. Инициализация `ans`
        // В Rust мы инициализируем вектор длины N, заполненный 1.
        let mut ans: Vec<i32> = vec![1; n];

        // 2. Структуры данных

        // mp: HashMap<Lake ID, Last Rain Day Index>
        let mut last_rain_day: HashMap<i32, usize> = HashMap::new();

        // st: BTreeSet<Dry Day Index>
        let mut dry_days: BTreeSet<usize> = BTreeSet::new();

        // 3. Главный цикл
        for i in 0..n {
            let lake_id = rains[i];

            if lake_id == 0 {
                // Если это сухой день, добавляем индекс дня в BTreeSet.
                dry_days.insert(i);
            } else {
                // Это дождливый день
                ans[i] = -1;

                if let Some(&last_day) = last_rain_day.get(&lake_id) {
                    // Озеро `lake_id` уже заполнялось ранее.
                    // Нам нужно найти ближайший сухой день (`dry_day_idx`)
                    // *после* предыдущего дождя (`last_day`).

                    // `st.ceiling(last_day)` находит наименьший элемент в `dry_days`
                    // >= `last_day`.
                    let dry_day_idx = dry_days.range(last_day..).copied().next();

                    if let Some(day_to_dry) = dry_day_idx {
                        // Успех: Нашли подходящий сухой день.

                        // В этот сухой день мы осушим озеро `lake_id`.
                        ans[day_to_dry] = lake_id;

                        // Удаляем этот сухой день из множества, так как он занят.
                        dry_days.remove(&day_to_dry);
                    } else {
                        // Провал: Нет доступного сухого дня, чтобы осушить озеро
                        // между двумя дождями.
                        return vec![];
                    }
                }

                // Обновляем день последнего дождя для этого озера.
                last_rain_day.insert(lake_id, i);
            }
        }

        // 4. Возвращаем результат
        ans
    }
}