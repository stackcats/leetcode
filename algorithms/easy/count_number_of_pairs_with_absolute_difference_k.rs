use std::collections::HashMap;

impl Solution {
    pub fn count_k_difference(nums: Vec<i32>, k: i32) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for n in nums.iter() {
            let target = *n + k;
            if map.contains_key(&target) {
                ans += map[&target];
            }
            let target = *n - k;
            if map.contains_key(&target) {
                ans += map[&target];
            }
            *map.entry(*n).or_insert(0) += 1;
        }

        ans
    }
}
