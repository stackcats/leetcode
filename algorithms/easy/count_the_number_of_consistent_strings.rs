use std::collections::HashSet;

impl Solution {
    pub fn count_consistent_strings(allowed: String, words: Vec<String>) -> i32 {
        let mut set = HashSet::new();
        for c in allowed.chars() {
            set.insert(c);
        }
        let mut ans = 0;
        for w in &words {
            let mut f = true;
            for c in w.chars() {
                if !set.contains(&c) {
                    f = false;
                    break;
                }
            }
            if f {
                ans += 1;
            }
        }
        ans
    }
}
