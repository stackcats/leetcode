use std::collections::HashMap;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut ct1 = HashMap::new();
        for c in s.chars() {
            *ct1.entry(c).or_insert(0) += 1;
        }
        let mut ct2 = HashMap::new();
        for c in target.chars() {
            *ct2.entry(c).or_insert(0) += 1;
        }

        let mut ans = i32::MAX;
        for (k, v) in ct2.into_iter() {
            if let Some(u) = ct1.get(&k) {
                ans = ans.min(u/v);
            } else {
                return 0;
            }
        }
        ans
    }
}
