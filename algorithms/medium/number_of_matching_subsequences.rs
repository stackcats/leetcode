use std::collections::HashMap;

impl Solution {
    pub fn num_matching_subseq(s: String, words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        for (i, c) in s.chars().enumerate() {
            let arr = map.entry(c).or_insert(vec![]);
            arr.push(i as i32);
        }
        let mut ans = 0;
        for word in &words {
            let mut last_index = -1;
            let mut is_subseq = true;
            for c in word.chars() {
                if !map.contains_key(&c) {
                    is_subseq = false;
                    break;
                }
                let tmp = last_index;
                for i in &map[&c] {
                    if *i > last_index {
                        last_index = *i;
                        break;
                    }
                }
                if tmp == last_index {
                    is_subseq = false;
                    break;
                }
            }
            if is_subseq {
                ans += 1;
            }
        }
        ans
    }
}
