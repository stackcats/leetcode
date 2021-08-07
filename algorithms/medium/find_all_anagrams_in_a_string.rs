// https://leetcode.com/problems/find-all-anagrams-in-a-string/

use std::collections::HashMap;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut dt = HashMap::new();
        for c in p.as_bytes() {
            let ct = dt.entry(*c).or_insert(0);
            *ct += 1;
        }

        let mut ans = Vec::new();

        let bs = s.as_bytes();
        let mut i = 0;
        let mut counter = dt.len();
        for j in 0..bs.len() {
            if let Some(ct) = dt.get_mut(&bs[j]) {
                *ct -= 1;
                if *ct == 0 {
                    counter -= 1;
                }
            }

            while counter == 0 {
                if j - i + 1 == p.len() {
                    ans.push(i as i32);
                }

                if let Some(ct) = dt.get_mut(&bs[i]) {
                    *ct += 1;
                    if *ct > 0 {
                        counter += 1;
                    }
                }
                i += 1;
            }
        }

        ans
    }
}
