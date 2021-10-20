use std::collections::HashMap;

impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for &n in &nums {
            if map.contains_key(&n) {
                continue;
            }
            let left = *map.get(&(n - 1)).unwrap_or(&0);
            let right = *map.get(&(n + 1)).unwrap_or(&0);
            let curr = left + right + 1;
            map.insert(n, curr);
            ans = ans.max(curr);
            map.insert(n - left, curr);
            map.insert(n + right, curr);
        }
        ans
    }
}
