impl Solution {
    pub fn minimum_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = nums.len() + 1;
        for i in 0..nums.len() {
            let mut or = 0;
            for j in i..nums.len() {
                or |= nums[j];
                if or >= k {
                    ans = ans.min(j - i + 1);
                }
            }
        }
        if ans == nums.len() + 1 {
            -1
        } else {
            ans as i32
        }
    }
}
