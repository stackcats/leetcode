use std::collections::HashSet;

impl Solution {
    pub fn find_repeated_dna_sequences(s: String) -> Vec<String> {
        if s.len() < 10 {
            return vec![];
        }
        let mut set = HashSet::new();
        let mut ans = HashSet::new();
        for i in (0..=s.len() - 10) {
            let t = &s[i..i + 10];
            if set.contains(t) {
                ans.insert(t);
            } else {
                set.insert(t);
            }
        }
        ans.into_iter().map(|x| x.to_owned()).collect()
    }
}
