impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut i = 0;
        let mut j = 0;
        while i < nums.len() && j < nums.len() {
            while i < nums.len() - 1 && nums[i] + 1 != nums[i + 1] {
                i += 1;
            }
            if i == nums.len() {
                break;
            }

            j = i + 1;

            while j < nums.len() && nums[j] - nums[j - 1] == (-1i32).pow((j - i + 1) as u32) {
                j += 1;
            }

            ans = ans.max(j - i);
            i = j - 1;
        }

        if ans == 1 {
            -1
        } else {
            ans as i32
        }
    }
}
