impl Solution {
    pub fn reverse_prefix(word: String, ch: char) -> String {
        let word: Vec<char> = word.chars().collect();

        let mut end_index = 0;
        while end_index < word.len() && word[end_index] != ch {
            end_index += 1;
        }

        if end_index == word.len() {
            return word.into_iter().collect();
        }

        let mut s = String::new();
        for i in (0..=end_index).rev() {
            s.push(word[i]);
        }
        for i in end_index + 1..word.len() {
            s.push(word[i]);
        }
        s
    }
}
