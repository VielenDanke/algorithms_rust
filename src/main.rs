mod array;
mod backtracking;
mod dynamic_programming;
mod graph;
mod hackerrank;
mod math;
mod strings;
mod structures;

// Импортируем BTreeMap для хранения (ключ, значение)
use std::collections::BTreeMap;
// Импортируем крейт Regex
use regex::Regex;
// --- Новые импорты для работы с файлами ---
use std::fs::File; // Для открытия файла
use std::io::{BufRead, BufReader}; // Для буферизованного чтения (построчно)
use std::path::Path; // Для работы с путями к файлам

///
/// Эта функция обрабатывает ОДНУ строку и обновляет карту (BTreeMap)
///
/// # Аргументы
/// * `line` - Ссылка на строку (&str), которую нужно проанализировать.
/// * `re` - Ссылка на скомпилированный Regex.
/// * `counts` - Мутабельная (изменяемая) ссылка на нашу карту BTreeMap.
///
fn process_line(line: &str, re: &Regex, counts: &mut BTreeMap<u32, u32>) {
    // .captures_iter() находит все совпадения (даже если их несколько в одной строке)
    for caps in re.captures_iter(line) {
        // .get(1) - получаем первую захватывающую группу (наши цифры)
        if let Some(id_match) = caps.get(1) {
            let id_str = id_match.as_str();

            // .parse::<u32>() пытается превратить строку "190" в число 190
            if let Ok(id_num) = id_str.parse::<u32>() {
                // --- Агрегация в BTreeMap ---
                // .entry(id_num).or_insert(0) получает ссылку на счетчик
                let count = counts.entry(id_num).or_insert(0);
                // *count += 1 - увеличиваем счетчик
                *count += 1;
            }
        }
    }
}

// --- Точка входа в программу ---
fn main() {
    println!(
        "{:?}",
        array::easy::task_3074::Solution::minimum_boxes(vec![5, 5, 5], vec![2, 4, 2, 7])
    );
}
