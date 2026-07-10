impl Solution {
    pub fn is_middle_element_unique(nums: Vec<i32>) -> bool {
        let m = nums.len() / 2;
        let t = nums[m];
        for (i, n) in nums.into_iter().enumerate() {
            if t == n && i != m {
                return false;
            }
        }
        true
    }
}
