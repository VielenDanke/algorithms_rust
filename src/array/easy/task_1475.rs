use std::collections::VecDeque;

pub struct Solution;

impl Solution {
    pub fn final_prices(mut prices: Vec<i32>) -> Vec<i32> {
        let n = prices.len();

        for i in 0..n {
            let mut current_price = prices[i];
            for j in i + 1..n {
                if prices[j] <= current_price {
                    current_price -= prices[j];
                    break;
                }
            }
            prices[i] = current_price;
        }
        prices
    }

    pub fn final_prices_faster(mut prices: Vec<i32>) -> Vec<i32> {
        let mut stack: Vec<(i32, usize)> = Vec::new();
        let n = prices.len();

        for i in 0..n {
            let v = prices[i];
            if stack.is_empty() || stack[stack.len() - 1].0 < v {
                stack.push((v, i));
            } else {
                while !stack.is_empty() && stack[stack.len() - 1].0 >= v {
                    let mut last_elem = stack.remove(stack.len() - 1);
                    prices[last_elem.1] = last_elem.0 - v;
                }
                stack.push((v, i));
            }
        }
        while let Some(elem) = stack.pop() {
            prices[elem.1] = elem.0;
        }
        prices
    }
}
