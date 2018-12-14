use std::collections::HashSet;

impl Solution {
    pub fn num_jewels_in_stones(j: String, s: String) -> i32 {
        let mut jewels = HashSet::new();
        for c in j.chars() {
            jewels.insert(c.clone());
        }

        let mut ans = 0;
        for c in s.chars().collect::<Vec<char>>().iter() {
            if jewels.contains(c) {
                ans += 1;
            } else {
                ans += 0;
            }
        }

        ans
    }
}
