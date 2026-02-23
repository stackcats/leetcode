use std::collections::HashSet;

impl Solution {
    pub fn smallest_absent(nums: Vec<i32>) -> i32 {
        let mut st = HashSet::new();
        let mut sum = 0;
        let mut len = nums.len() as f64;
        for n in nums {
            sum += n;
            st.insert(n);
        }
        let mut avg = (sum as f64 / len).floor() as i32 + 1;
        avg = avg.max(1);
        while st.contains(&avg) {
            avg += 1;
        }
        avg
    }
}
