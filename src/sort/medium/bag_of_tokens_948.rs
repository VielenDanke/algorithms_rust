pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
    if tokens.len() == 0 {
        return 0;
    }
    tokens.sort();

    let (mut result, mut points, mut left, mut right) = (0, 0, 0 as usize, tokens.len() - 1 as usize);

    while left <= right {
        if power >= tokens[left] {
            power -= tokens[left];
            left += 1;
            points += 1;
            result = std::cmp::max(result, points);
        } else if points > 0 {
            points -= 1;
            power += tokens[right];
            right -= 1;
        } else {
            break;
        }
    }
    result
}
