use std::collections::HashMap;

impl Solution {
    pub fn majority_frequency_group(s: String) -> String {
        let mut mp = HashMap::new();
        for c in s.chars() {
            *mp.entry(c).or_insert(0) += 1;
        }

        let mut ct = HashMap::new();
        let mut freq = 0;
        let mut ans = String::new();
        for (c, f) in mp {
            ct.entry(f).or_insert(String::new()).push(c);
            let v = ct.get(&f).unwrap();
            if v.len() > ans.len() || (v.len() == ans.len() && f > freq) {
                freq = f;
                ans = v.to_string();
            }
        }

        ans
    }
}
