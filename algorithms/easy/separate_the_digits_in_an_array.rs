impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = Vec::new();
        for n in nums {
            let s = n.to_string();
            for b in s.as_bytes() {
                ans.push((b - b'0') as i32);
            }
        }
        ans
    }
}
