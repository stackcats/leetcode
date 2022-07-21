impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum: i32 = stones.iter().sum();
        let target = (sum / 2) as usize;
        let mut dp = vec![0; target + 1];
        for stone in stones {
            for j in (stone as usize..=target).rev() {
                dp[j] = dp[j].max(dp[j-stone as usize] + stone);
            }
        }

        sum - 2 * dp[target]
    }
}
