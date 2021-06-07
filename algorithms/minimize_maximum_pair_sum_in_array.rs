impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let mut ans = 0;
        let mut i = 0;
        let mut j = nums.len() - 1;
        while i < j {
            ans = ans.max(nums[i] + nums[j]);
            i += 1;
            j -= 1;
        }
        ans
    }
}
