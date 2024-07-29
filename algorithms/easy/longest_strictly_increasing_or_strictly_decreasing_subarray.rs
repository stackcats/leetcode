impl Solution {
    pub fn longest_monotonic_subarray(nums: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut lt = 0;
        let mut gt = 0;
        for i in 1..nums.len() {
            if nums[i] > nums[i - 1] {
                max = max.max(i - gt);
                gt = i;
            } else if nums[i] < nums[i - 1] {
                max = max.max(i - lt);
                lt = i;
            } else {
                max = max.max(i - gt);
                gt = i;
                max = max.max(i - lt);
                lt = i;
            }
        }
        max.max(nums.len() - lt).max(nums.len() - gt) as i32
    }
}
