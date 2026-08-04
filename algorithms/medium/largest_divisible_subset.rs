impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();
        let n = nums.len();
        let mut dp = vec![1; n];
        let mut prev: Vec<_> = (0..n).collect();
        let mut ma = 0;
        let mut k = 0;
        for i in 0..n {
            for j in i + 1..n {
                if nums[j] % nums[i] != 0 || dp[j] > dp[i] + 1 {
                    continue;
                }
                dp[j] = dp[i] + 1;
                prev[j] = i;
                if ma < dp[j] {
                    ma = dp[j];
                    k = j;
                }
            }
        }

        let mut ans = Vec::new();
        while prev[k] != k {
            ans.push(nums[k]);
            k = prev[k];
        }
        ans.push(nums[k]);
        ans
    }
}
