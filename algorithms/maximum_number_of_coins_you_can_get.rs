impl Solution {
    pub fn max_coins(mut piles: Vec<i32>) -> i32 {
        piles.sort_by(|a, b| b.cmp(a));
        let mut sum = 0;
        for i in (1..(piles.len() * 2 / 3)).step_by(2) {
            sum += piles[i];
        }
        sum
    }
}
