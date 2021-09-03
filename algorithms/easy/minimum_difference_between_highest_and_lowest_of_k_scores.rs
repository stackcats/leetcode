impl Solution {
    pub fn minimum_difference(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort();

        let k = k as usize;
        if nums.len() <= k {
            return nums[nums.len() - 1] - nums[0];
        }

        let mut ans = i32::MAX;
        for i in k - 1..nums.len() {
            ans = ans.min(nums[i] - nums[i + 1 - k]);
        }

        ans
    }
}
