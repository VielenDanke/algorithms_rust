use std::collections::HashMap;

struct Spreadsheet {
    value_map: HashMap<String, i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Spreadsheet {

    fn new(rows: i32) -> Self {
        Spreadsheet {
            value_map: HashMap::new()
        }
    }

    fn set_cell(&mut self, cell: String, value: i32) {
        self.value_map.insert(cell, value);
    }

    fn reset_cell(&mut self, cell: String) {
        self.value_map.remove(&cell);
    }

    fn get_value(&mut self, formula: String) -> i32 {
        let formula = &formula[1..];
        let (left, right) = formula
            .split_once('+')
            .unwrap();

        let left_value = if left.chars().next().unwrap().is_alphabetic() {
            *self.value_map.get(left).unwrap_or(&0)
        } else {
            left.parse::<i32>().unwrap_or(0)
        };

        let right_value = if right.chars().next().unwrap().is_alphabetic() {
            *self.value_map.get(right).unwrap_or(&0)
        } else {
            right.parse::<i32>().unwrap_or(0)
        };

        left_value + right_value
    }
}