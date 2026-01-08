impl Solution {
    pub fn max_dot_product(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let mut dp = vec![vec![i32::MIN; nums2.len() + 1]; nums1.len() + 1];

        dp[0][0] = 0;

        for i in 1..=nums1.len() {
            for j in 1..=nums2.len() {
                dp[i][j] = [
                    nums1[i - 1] * nums2[j - 1] + dp[i - 1][j - 1].max(0),
                    dp[i - 1][j],
                    dp[i][j - 1],
                ]
                .into_iter()
                .max()
                .unwrap()
            }
        }
        dp[nums1.len()][nums2.len()]
    }
}
