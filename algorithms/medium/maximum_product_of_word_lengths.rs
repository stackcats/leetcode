impl Solution {
    pub fn max_product(words: Vec<String>) -> i32 {
        let mut ans = 0;
        let mut set = vec![0_u32; words.len()];
        for (i, w) in words.iter().enumerate() {
            for b in w.as_bytes() {
                set[i] |= 1 << ((b - b'a') as u32);
            }
        }
        for i in 0..words.len() - 1 {
            for j in i + 1..words.len() {
                if set[i] & set[j] == 0 {
                    ans = ans.max(words[i].len() * words[j].len());
                }
            }
        }

        ans as i32
    }
}
