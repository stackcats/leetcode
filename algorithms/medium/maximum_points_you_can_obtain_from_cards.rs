impl Solution {
    pub fn max_score(card_points: Vec<i32>, k: i32) -> i32 {
        let k = card_points.len() - k as usize;
        let mut curr = card_points[k..].iter().sum::<i32>();
        let mut ans = curr;
        for j in k..card_points.len() {
            curr -= card_points[j];
            curr += card_points[j - k];
            ans = ans.max(curr);
        }
        ans
    }
}
