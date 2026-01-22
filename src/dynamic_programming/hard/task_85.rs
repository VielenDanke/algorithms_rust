use std::cmp;

pub struct Solution;

impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        if matrix.is_empty() {
            return 0;
        }

        let cols = matrix[0].len();
        // We use cols + 1 to add a '0' sentinel at the end.
        // This ensures the stack is always flushed at the end of the histogram loop.
        let mut heights = vec![0; cols + 1];
        let mut max_area = 0;

        for row in matrix {
            // 1. Update the histogram heights for the current row
            for i in 0..cols {
                if row[i] == '1' {
                    heights[i] += 1;
                } else {
                    heights[i] = 0;
                }
            }

            // 2. Calculate Largest Rectangle in Histogram (Monotonic Stack)
            let mut stack: Vec<usize> = Vec::new();

            for i in 0..=cols {
                // While current bar is lower than the bar at stack top, pop and calculate area
                while let Some(&top) = stack.last() {
                    if heights[i] >= heights[top] {
                        break;
                    }
                    stack.pop();

                    let h = heights[top];
                    // Width is determined by the current index and the new top of stack
                    let w = if let Some(&prev) = stack.last() {
                        i - prev - 1
                    } else {
                        i
                    };

                    max_area = cmp::max(max_area, h * w);
                }
                stack.push(i);
            }
        }

        max_area as i32
    }
}