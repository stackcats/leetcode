impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i64 {
        let mut ans = 0;

        for i in 1..nums.len() {
            ans += (nums[i - 1] - nums[i]).max(0) as i64;
        }

        ans
    }
}
