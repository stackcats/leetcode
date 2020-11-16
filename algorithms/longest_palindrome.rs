use std::collections::HashMap;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut map = HashMap::new();
        for c in s.chars() {
            *map.entry(c).or_insert(0) += 1;
        }
        let mut has_odd = 0;
        let mut ans = 0;
        for v in map.values() {
            if *v % 2 != 0 {
                has_odd = 1;
            }
            ans += v / 2 * 2;
        }
        ans + has_odd
    }
}
