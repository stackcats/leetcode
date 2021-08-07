impl Solution {
    pub fn find_max_consecutive_ones(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut curr = 0;
        for n in &nums {
            if *n == 1 {
                curr += 1;
            } else {
                ans = ans.max(curr);
                curr = 0;
            }
        }
        ans.max(curr)
    }
}
