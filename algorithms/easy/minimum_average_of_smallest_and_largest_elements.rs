impl Solution {
    pub fn minimum_average(mut nums: Vec<i32>) -> f64 {
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut ans: f64 = f64::MAX;
        while i < j {
            let n = nums[i] + nums[j];
            ans = ans.min(n as f64 / 2.0);
            i += 1;
            j -= 1;
        }
        ans
    }
}
