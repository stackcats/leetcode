impl Solution {
    pub fn find_duplicate(nums: Vec<i32>) -> i32 {
        let mut slow = 0;
        let mut fast = 0;
        slow = nums[slow] as usize;
        fast = nums[nums[fast] as usize] as usize;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[nums[fast] as usize] as usize;
        }

        fast = 0;
        while slow != fast {
            slow = nums[slow] as usize;
            fast = nums[fast] as usize;
        }
        slow as i32
    }
}
