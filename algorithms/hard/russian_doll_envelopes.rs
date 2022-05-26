impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        envelopes.sort_by_key(|k| (k[0], -k[1]));
        let mut dp = Vec::new();
        for each in envelopes {
            let i = dp.binary_search(&each[1]).unwrap_or_else(|x| x);
            if i == dp.len() {
                dp.push(each[1]);
            } else {
                dp[i] = each[1];
            }
        }
        dp.len() as i32
    }
}
