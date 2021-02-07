use std::collections::HashMap;

impl Solution {
    pub fn sum_of_unique(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for n in &nums {
            let ct = map.entry(n).or_insert(0);
            if *ct == 0 {
                ans += *n;
            } else if *ct == 1 {
                ans -= *n
            }
            *ct += 1;
        }
        ans
    }
}
