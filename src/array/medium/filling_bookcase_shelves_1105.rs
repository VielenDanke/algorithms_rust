use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn min_height_shelves(books: Vec<Vec<i32>>, shelf_width: i32) -> i32 {
        fn arrange_books(books: &Vec<Vec<i32>>, max_shelf_width: i32) -> i32 {
            let n = books.len();
            let mut min_heights = vec![i32::MAX; n + 1];
            min_heights[0] = 0;

            for book_index in 1..=n {
                let mut current_shelf_height = 0;
                let mut current_shelf_width = 0;

                for last_book in (0..book_index).rev() {
                    let (current_book_thickness, current_book_height) = (books[last_book][0], books[last_book][1]);

                    if current_shelf_width + current_book_thickness > max_shelf_width {
                        break;
                    }

                    current_shelf_width += current_book_thickness;
                    current_shelf_height = current_shelf_height.max(current_book_height);

                    let current_arrangement_height = min_heights[last_book] + current_shelf_height;
                    min_heights[book_index] = min_heights[book_index].min(current_arrangement_height);
                }
            }

            min_heights[n]
        }

        arrange_books(&books, shelf_width)
    }
}
