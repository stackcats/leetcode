use std::i32;
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        if nums[start as usize] == target {
            return 0;
        }
        let mut i = start - 1;
        let mut left = i32::MAX;
        while i >= 0 {
            if nums[i as usize] == target {
                left = start - i;
                break;
            }
            i -= 1;
        }
        let mut right = i32::MAX;
        i = start + 1;
        while (i as usize) < nums.len() {
            if nums[i as usize] == target {
                right = i - start;
                break;
            }
            i += 1;
        }
        left.min(right)
    }
}
