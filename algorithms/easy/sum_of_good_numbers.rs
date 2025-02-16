impl Solution {
    pub fn sum_of_good_numbers(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let k = k as usize;
        let len = nums.len();
        for (i, &n) in nums.iter().enumerate() {
            if i + k < len && n <= nums[i + k] {
                continue;
            }
            if i - k < len && n <= nums[i - k] {
                continue;
            }
            ans += n;
        }
        ans
    }
}

