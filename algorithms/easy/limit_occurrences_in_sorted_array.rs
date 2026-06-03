impl Solution {
    pub fn limit_occurrences(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut prev = i32::MAX;
        let mut t = 0;
        for n in nums {
            if n != prev {
                prev = n;
                t = k;
            }
            if t > 0 {
                ans.push(n);
                t -= 1;
            }
        }
        ans
    }
}
