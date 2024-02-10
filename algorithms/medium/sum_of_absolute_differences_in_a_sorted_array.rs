impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let mut left = 0;
        let mut right: i32 = nums.iter().sum();
        let mut ans = vec![0; nums.len()];
        for (i, n) in nums.iter().enumerate() {
            right -= n;
            ans[i] = right - left + n * (2 * i - nums.len() + 1) as i32;
            left += n;
        }
        ans
    }
}
