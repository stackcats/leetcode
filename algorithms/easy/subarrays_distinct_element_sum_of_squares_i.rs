impl Solution {
    pub fn sum_counts(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..nums.len() {
            let mut counter = [0; 101];
            let mut ct = 0;
            for j in i..nums.len() {
                let c = nums[j] as usize;
                counter[c] += 1;
                if counter[c] == 1 {
                    ct += 1;
                }
                ans += ct * ct;
            }
        }
        ans
    }
}
