impl Solution {
    pub fn final_prices(prices: Vec<i32>) -> Vec<i32> {
        let mut arr = Vec::new();
        for i in 0..prices.len() {
            let mut j = i + 1;
            let mut hasDiscount = false;
            while j < prices.len() {
                if prices[i] >= prices[j] {
                    hasDiscount = true;
                    break;
                }
                j += 1;
            }
            if hasDiscount {
                arr.push(prices[i] - prices[j]);
            } else {
                arr.push(prices[i]);
            }
        }
        arr
    }
}
