use regex::Regex;
use std::collections::HashMap;

impl Solution {
    pub fn count_word_occurrences(chunks: Vec<String>, queries: Vec<String>) -> Vec<i32> {
        let s = chunks.join("");
        let re = Regex::new(r"(^\-|\-$|\-\s|\s\-|\-\-+|\s)").unwrap();
        let v: Vec<&str> = re.split(&s).collect();
        let mut mp = HashMap::new();
        for t in &v {
            *mp.entry(t.trim_matches('-').to_string()).or_insert(0) += 1;
        }
        let mut ans = vec![0; queries.len()];
        for (i, q) in queries.iter().enumerate() {
            ans[i] = *mp.get(q).unwrap_or(&0);
        }
        ans
    }
}
