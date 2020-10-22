impl Solution {
    pub fn string_matching(mut words: Vec<String>) -> Vec<String> {
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut ans = Vec::new();
        for i in 0..words.len() {
            let mut flag = false;
            for j in (i + 1)..words.len() {
                if words[j].contains(words[i].as_str()) {
                    flag = true;
                    break;
                }
            }
            if flag {
                ans.push(words[i].to_string());
                continue;
            }
        }
        ans
    }
}
