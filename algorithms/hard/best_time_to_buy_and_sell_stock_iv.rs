impl Solution {
    pub fn max_profit(k: i32, prices: Vec<i32>) -> i32 {
        if prices.len() == 0 || k == 0 {
            return 0;
        }
        let k = k as usize;
        let mut dp = vec![vec![0; prices.len()]; k + 1];
        for i in 1..=k {
            let mut buy_price = -prices[0];
            for j in 1..prices.len() {
                dp[i][j] = dp[i][j - 1].max(prices[j] + buy_price);
                buy_price = buy_price.max(dp[i - 1][j - 1] - prices[j]);
            }
        }

        dp[k][prices.len() - 1]
    }
}
