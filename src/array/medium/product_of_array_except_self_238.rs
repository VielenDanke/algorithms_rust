pub fn product_except_self(mut nums: Vec<i32>) -> Vec<i32> {
    let mut product = 1;
    let mut zeroes = 0;

    for num in nums.iter() {
        if *num == 0 {
            zeroes += 1;
            continue;
        }
        product *= num;
    }
    if zeroes > 1 {
        for num in &mut nums {
            *num = 0;
        }
    } else if zeroes == 1 {
        for num in &mut nums {
            if *num != 0 {
                *num = 0;
            } else {
                *num = product;
            }
        }
    } else {
        for num in &mut nums {
            *num = product / *num;
        }
    }
    nums
}
