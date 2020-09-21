impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut h1 = -1;
        let mut h2 = -2;
        for i in 0..nums.len() {
            if nums[i] >= h1 {
                h2 = h1;
                h1 = nums[i];
            } else if nums[i] > h2 {
                h2 = nums[i];
            }
        }
        (h1 - 1) * (h2 - 1)
    }
}
