impl Solution {
    pub fn count_odds(low: i32, high: i32) -> i32 {
        if low % 2 == 1 && high % 2 == 1 {
            return (high - low) / 2 + 1;
        }
        if low % 2 == 1 || high % 2 == 1 {
            // change high or low to even by minus 1
            return 1 + (high - low - 1) / 2;
        }
        (high - low) / 2
    }
}
