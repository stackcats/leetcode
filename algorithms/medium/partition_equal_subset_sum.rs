impl Solution {
    pub fn can_partition(nums: Vec<i32>) -> bool {
        let mut sum = nums.iter().sum::<i32>() as usize;
        
        if sum % 2 != 0 {
            return false;
        }

        sum /= 2;
        
        let mut dp = vec![false; sum + 1];
        
        dp[0] = true;
        
        for n in nums {
            for j in (n as usize..=sum).rev() {
                dp[j] = dp[j] || dp[j - n as usize];
            }
        }

        dp[sum]
    }
}
