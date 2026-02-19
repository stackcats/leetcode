impl Solution {
    pub fn map_word_weights(words: Vec<String>, weights: Vec<i32>) -> String {
        let mut ans = String::new();
        for w in words {
            let mut sum = 0;
            for i in w.as_bytes() {
                let j = (*i as u8 - b'a') as usize;
                sum += weights[j];
            }
            let r = (sum % 26) as u8;
            let c = (b'a' + 26 - r - 1) as char;
            ans.push(c);
        }
        ans
    }
}
