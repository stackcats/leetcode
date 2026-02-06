impl Solution {
    pub fn min_removal(mut nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        nums.sort();
        let mut j = 0;
        let mut ans = usize::MAX;
        for i in 0..nums.len() {
            while j < nums.len() && (nums[j] as usize) <= (nums[i] as usize) * k {
                j += 1;
            }
            ans = ans.min(nums.len() - j + i);
        }

        ans as _
    }
}
