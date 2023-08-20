impl Solution {
    pub fn count_pairs(mut nums: Vec<i32>, target: i32) -> i32 {
        nums.sort();
        let mut ct = 0;
        let mut l = 0;
        let mut r = nums.len() - 1;
        while l < r {
            if nums[l] + nums[r] < target {
                ct += r - l;
                l += 1;
            } else {
                r -= 1;
            }
        }
        ct as i32
    }
}
