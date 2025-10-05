/// Структура, содержащая решение задачи Pacific Atlantic Water Flow.
pub struct Solution;

impl Solution {
    /// Находит координаты ячеек, из которых вода может стекать как в Тихий,
    /// так и в Атлантический океаны, используя DFS.
    ///
    /// Логика: Начинаем DFS с "береговых" ячеек (края сетки) и движемся
    /// вглубь суши. Вода может двигаться из ячейки (x, y) в (i, j), только если
    /// height[x][y] >= height[i][j] (так как мы идем *против* потока воды).
    ///
    /// # Аргументы
    /// * `heights`: Матрица высот.
    ///
    /// # Возвращает
    /// Вектор координат `[r, c]`, соответствующих условию.
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let rows = heights.len();
        let cols = heights.first().map_or(0, |r| r.len());

        if rows == 0 || cols == 0 {
            return Vec::new();
        }

        // Массивы для отслеживания ячеек, из которых можно добраться
        // до Тихого и Атлантического океанов, соответственно.
        let mut pacific_reachable = vec![vec![false; cols]; rows];
        let mut atlantic_reachable = vec![vec![false; cols]; rows];

        // 1. Запуск DFS с берегов Тихого океана:
        //    Левый столбец (c = 0) и верхняя строка (r = 0).
        //    Rust-итераторы делают код более лаконичным, чем циклы for в Java.
        for r in 0..rows {
            Self::dfs(r, 0, rows, cols, &heights, &mut pacific_reachable);
        }
        for c in 0..cols {
            Self::dfs(0, c, rows, cols, &heights, &mut pacific_reachable);
        }

        // 2. Запуск DFS с берегов Атлантического океана:
        //    Правый столбец (c = cols - 1) и нижняя строка (r = rows - 1).
        for r in 0..rows {
            Self::dfs(r, cols - 1, rows, cols, &heights, &mut atlantic_reachable);
        }
        for c in 0..cols {
            Self::dfs(rows - 1, c, rows, cols, &heights, &mut atlantic_reachable);
        }

        // 3. Формирование окончательного результата.
        let mut result = Vec::new();
        for r in 0..rows {
            for c in 0..cols {
                // Ячейка должна быть достижима из обоих океанов.
                if pacific_reachable[r][c] && atlantic_reachable[r][c] {
                    // Используем .push(vec![...]) для добавления координат.
                    result.push(vec![r as i32, c as i32]);
                }
            }
        }

        result
    }

    /// Рекурсивная функция поиска в глубину (Depth First Search).
    ///
    /// # Аргументы
    /// * `r`, `c`: Текущие координаты (строка, столбец).
    /// * `rows`, `cols`: Размеры сетки.
    /// * `heights`: Ссылка на матрицу высот.
    /// * `visited`: Ссылка на массив достижимости, который мы обновляем.
    fn dfs(
        r: usize,
        c: usize,
        rows: usize,
        cols: usize,
        heights: &Vec<Vec<i32>>,
        visited: &mut Vec<Vec<bool>>,
    ) {
        // Если ячейка уже посещена, выходим.
        if visited[r][c] {
            return;
        }

        // Помечаем текущую ячейку как достижимую.
        visited[r][c] = true;

        // Определяем возможные направления движения: (вниз, вверх, вправо, влево).
        // Используем знаковое целое (i32) для безопасного вычисления смещений.
        let directions: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

        let current_height = heights[r][c];

        for (dr, dc) in directions.iter() {
            let next_r = (r as i32) + dr;
            let next_c = (c as i32) + dc;

            // 1. Проверка границ
            if next_r < 0 || next_r >= rows as i32 || next_c < 0 || next_c >= cols as i32 {
                continue;
            }

            // Безопасное преобразование в usize после проверки границ.
            let next_r_u = next_r as usize;
            let next_c_u = next_c as usize;

            // 2. Проверка условия потока (движемся от океана к суше):
            // Вода, стекающая из (next_r_u, next_c_u), может попасть в (r, c),
            // если высота в следующей ячейке НЕ МЕНЬШЕ текущей.
            if heights[next_r_u][next_c_u] >= current_height {
                // Рекурсивный вызов DFS
                Self::dfs(next_r_u, next_c_u, rows, cols, heights, visited);
            }
        }
    }
}