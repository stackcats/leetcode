impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, mut power: i32) -> i32 {
        if tokens.len() == 0 {
            return 0;
        }
        tokens.sort();
        let mut i = 0;
        let mut j = tokens.len() - 1;
        let mut score = 0;
        while i <= j {
            if power >= tokens[i] {
                score += 1;
                power -= tokens[i];
                i += 1;
            } else if score > 0 && i < j {
                power += tokens[j];
                j -= 1;
                score -= 1;
            } else {
                break;
            }
        }
        score
    }
}
