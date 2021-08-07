impl Solution {
    pub fn build_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for n in &nums {
            ans.push(nums[*n as usize]);
        }
        ans
    }
}
