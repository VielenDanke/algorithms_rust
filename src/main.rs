mod math;
mod dynamic_programming;
mod array;
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

    // --- 1. Определяем все переменные ---
    let file_path = "logs.txt"; // Имя нашего файла

    // Тот же шаблон Regex
    let pattern = r"service_id:\s*(\d+)";

    // Компилируем Regex один раз. .expect() вызовет панику, если шаблон неверный
    let re = Regex::new(pattern).expect("Ошибка компиляции Regex");

    // Создаем пустую BTreeMap, которую будем наполнять
    let mut id_counts: BTreeMap<u32, u32> = BTreeMap::new();

    println!("Читаем файл: {}...", file_path);

    // --- 2. Открываем файл ---
    // File::open может вернуть ошибку, поэтому используем .expect()
    let file = File::open(Path::new(file_path))
        .expect("Не удалось открыть файл input.log");

    // Оборачиваем файл в BufReader для эффективного построчного чтения
    let reader = BufReader::new(file);

    // --- 3. Читаем файл построчно ---
    // reader.lines() возвращает итератор по строкам (точнее, по Result<String>)
    for line_result in reader.lines() {

        // .expect() здесь нужен, т.к. чтение строки тоже может вызвать ошибку
        let line = line_result.expect("Не удалось прочитать строку");

        // Передаем строку, Regex и карту в нашу функцию-обработчик
        process_line(&line, &re, &mut id_counts);
    }

    // --- 4. Печатаем финальный результат ---
    println!("\n✅ Результат подсчета (BTreeMap):");
    println!("{:#?}", id_counts);

    println!("\nПодробный отчет:");
    if id_counts.is_empty() {
        println!("  'service_id' не найдены.");
    } else {
        // Итерируем по BTreeMap (он уже отсортирован!)
        for (service_id, count) in &id_counts {
            println!("  ID {}: встречается {} раз(а)", service_id, count);
        }
    }
}
