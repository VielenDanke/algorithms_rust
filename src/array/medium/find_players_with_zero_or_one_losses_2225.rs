pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut scores = vec![0; 100001];

    for m in matches {
        let winner = m[0] as usize;
        let looser = m[1] as usize;

        if scores[winner] == 0 {
            scores[winner] = 1;
        }
        if scores[looser] == 0 {
            scores[looser] = 1;
        }
        scores[looser] += 1;
    }
    let mut zeroes: Vec<i32> = Vec::new();
    let mut ones: Vec<i32> = Vec::new();

    for (player, &score) in scores.iter().enumerate() {
        if score == 1 {
            zeroes.push(player as i32)
        } else if score == 2 {
            ones.push(player as i32)
        }
    }
    vec![zeroes, ones]
}
