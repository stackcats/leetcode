impl Solution {
    pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
        let mut arr = Vec::new();
        for i in (0..nums.len()).step_by(2) {
            for j in 0..nums[i] {
                arr.push(nums[i + 1]);
            }
        }
        arr
    }
}
