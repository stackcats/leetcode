impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::with_capacity(nums.len());
        let mut prefix = 1;
        for &n in &nums {
            ans.push(prefix);
            prefix *= n;
        }

        let mut suffix = 1;
        for (i, n) in nums.iter().enumerate().rev() {
            ans[i] *= suffix;
            suffix *= nums[i];
        }
        ans
    }
}
