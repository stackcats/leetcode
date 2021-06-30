impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        let mut left = 0;
        let mut curr_num_of_zeros = 0;
        let mut ans = 0;
        for right in 0..nums.len() {
            if nums[right] == 0 {
                curr_num_of_zeros += 1;
            }
            while curr_num_of_zeros > k {
                if nums[left] == 0 {
                    curr_num_of_zeros -= 1;
                }
                left += 1;
            }
            ans = ans.max((right - left + 1) as i32);
        }
        ans
    }
}
