pub struct Solution;

impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        
        let mut max_tops = HashMap::new();
        let mut max_bottoms = HashMap::new();
        let mut max_tops_same = 0;
        let mut max_bottoms_same = 0;
        let mut max_tops_same_num = 0;
        let mut max_bottoms_same_num = 0;

        for (&top, &bottom) in tops.iter().zip(&bottoms) {
            let top_count = max_tops.entry(top).or_insert(0);
            *top_count += 1;
            if *top_count > max_tops_same {
                max_tops_same = *top_count;
                max_tops_same_num = top;
            }

            let bottom_count = max_bottoms.entry(bottom).or_insert(0);
            *bottom_count += 1;
            if *bottom_count > max_bottoms_same {
                max_bottoms_same = *bottom_count;
                max_bottoms_same_num = bottom;
            }
        }

        if max_tops_same > max_bottoms_same {
            for (top, bottom) in tops.iter().zip(&bottoms) {
                if *top != max_tops_same_num && *bottom != max_tops_same_num {
                    return -1;
                }
            }
            return (tops.len() as i32) - (max_tops_same);
        } else if max_tops_same < max_bottoms_same {
            for (top, bottom) in tops.iter().zip(&bottoms) {
                if *bottom != max_bottoms_same_num && *top != max_bottoms_same_num {
                    return -1;
                }
            }
            return (bottoms.len() as i32) - (max_bottoms_same);
        } else {
            let mut found = true;
            for (top, bottom) in tops.iter().zip(&bottoms) {
                if *top != max_tops_same_num && *bottom != max_tops_same_num {
                    found = false;
                    break;
                }
            }
            if found {
                return (tops.len() as i32) - (max_tops_same);
            }

            found = true;
            for (top, bottom) in tops.iter().zip(&bottoms) {
                if *bottom != max_bottoms_same_num && *top != max_bottoms_same_num {
                    found = false;
                    break;
                }
            }
            if found {
                return (bottoms.len() as i32) - (max_bottoms_same as i32);
            }

            return -1;
        }
    }
}