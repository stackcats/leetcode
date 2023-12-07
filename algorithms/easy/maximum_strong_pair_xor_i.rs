impl Solution {
    pub fn maximum_strong_pair_xor(mut nums: Vec<i32>) -> i32 {
        nums.sort_unstable();
        let mut ans = 0;
        for i in 0..nums.len() {
            let x = nums[i];
            for j in i + 1..nums.len() {
                let y = nums[j];
                if y > 2 * x {
                    break;
                }
                ans = ans.max(x ^ y);
            }
        }
        ans
    }
}
