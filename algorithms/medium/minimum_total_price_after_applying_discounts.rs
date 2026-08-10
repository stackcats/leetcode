impl Solution {
    pub fn min_price(mut prices: Vec<i32>, mut discounts: Vec<i32>) -> f64 {
        prices.sort_by(|a, b| b.cmp(a));
        discounts.sort_by(|a, b| b.cmp(a));
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0.0;
        while i < prices.len() && j < discounts.len() {
            ans += (prices[i] as f64) * (100.0 - discounts[j] as f64) / 100.0;
            i += 1;
            j += 1;
        }
        while i < prices.len() {
            ans += prices[i] as f64;
            i += 1;
        }
        ans
    }
}
