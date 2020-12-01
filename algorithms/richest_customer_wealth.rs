impl Solution {
    pub fn maximum_wealth(accounts: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        for each in &accounts {
            let sum: i32 = each.iter().sum();
            ans = sum.max(ans);
        }
        ans
    }
}
