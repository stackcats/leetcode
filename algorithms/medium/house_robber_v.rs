impl Solution {
    pub fn rob(nums: Vec<i32>, colors: Vec<i32>) -> i64 {
        let mut prev_prev = 0;
        let mut prev = 0;
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut curr = nums[i] as i64 + prev_prev;
            if i > 0 && colors[i] != colors[i - 1] {
                curr = curr.max(prev + nums[i] as i64);
            } else {
                curr = curr.max(prev);
            }
            prev_prev = prev;
            prev = curr;
        }
        prev
    }
}
