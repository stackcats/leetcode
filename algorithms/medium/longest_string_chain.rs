use std::collections::HashMap;

impl Solution {
    pub fn longest_str_chain(mut words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 1;

        words.sort_by_key(|w| w.len());
        for w in &words {
            let mut curr_len = 1;
            for i in 0..w.len() {
                let mut t = w.to_string();
                t.remove(i);
                let len = map.get(&t).unwrap_or(&0);
                curr_len = curr_len.max(len + 1);
                ans = ans.max(curr_len);
                map.insert(w, curr_len);
            }
        }
        
        ans
    }
}
