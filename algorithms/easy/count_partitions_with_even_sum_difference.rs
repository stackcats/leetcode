impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let mut right: i32 = nums.iter().sum();
        let mut left = 0;
        let mut ct = 0;
        for i in 0..nums.len() - 1 {
            let n = nums[i];
            left += n;
            right -= n;
            let diff = (left - right).abs();
            if diff % 2 == 0 {
                ct += 1;
            }
        }
        ct
    }
}
