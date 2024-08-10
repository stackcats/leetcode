impl Solution {
    pub fn get_longest_subsequence(words: Vec<String>, groups: Vec<i32>) -> Vec<String> {
        let mut ans = Vec::new();
        let mut last = groups[0] ^ 1;
        for i in 0..groups.len() {
            if groups[i] != last {
                ans.push(words[i].to_string());
                last = groups[i];
            }
        }
        ans
    }
}
