impl Solution {
    pub fn max_sum_after_partitioning(arr: Vec<i32>, k: i32) -> i32 {
        let mut dp = vec![0; arr.len() + 1];
        for i in 1..=arr.len() {
            let mut localMax = 0;
            for j in 1..=i.min(k as usize) {
                localMax = localMax.max(arr[i - j]);
                dp[i] = dp[i].max(dp[i - j] + (j as i32) * localMax);
            }
        }
        dp[arr.len()]
    }
}
