use std::collections::HashMap;

impl Solution {
    pub fn num_identical_pairs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count: HashMap<i32, i32> = HashMap::new();
        for n in &nums {
            if count.contains_key(n) {
                ans += count[n];
                count.insert(*n, count[n] + 1);
            } else {
                count.insert(*n, 1);
            }
        }
        ans
    }
}
