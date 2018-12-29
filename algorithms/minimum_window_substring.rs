// https://leetcode.com/problems/minimum-window-substring/

use std::collections::HashMap;

impl Solution {
    pub fn min_window(s: String, t: String) -> String {
        let mut dt = HashMap::new();

        for b in t.as_bytes() {
            let ct = dt.entry(*b).or_insert(0);
            *ct += 1;
        }

        let mut min_len = s.len() + 1;
        let mut begin_index = 0;
        let mut count = dt.len() as i32;

        let bs = s.as_bytes();
        let mut i = 0;
        for j in 0..bs.len() {
            if let Some(ct) = dt.get_mut(&bs[j]) {
                *ct -= 1;
                if *ct == 0 {
                    count -= 1;
                }
            }

            while count == 0 {
                if j - i < min_len {
                    min_len = j - i;
                    begin_index = i;
                }

                if let Some(ct) = dt.get_mut(&bs[i]) {
                    *ct += 1;
                    if *ct > 0 {
                        count += 1;
                    }
                }

                i += 1;
            }
        }

        if min_len > s.len() {
            return "".to_string();
        }

        std::str::from_utf8(&bs[begin_index..begin_index + min_len + 1])
            .unwrap()
            .to_string()
    }
}
