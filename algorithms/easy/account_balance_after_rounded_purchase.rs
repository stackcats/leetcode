impl Solution {
    pub fn account_balance_after_purchase(purchase_amount: i32) -> i32 {
        let roundedAmount = (purchase_amount as f64 / 10.0).round() * 10.0;
        100 - (roundedAmount as i32)
    }
}
