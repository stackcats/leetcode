impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut mp = [0; 26];
        for c in word.bytes() {
            let i = c - b'a';
            mp[i as usize] += 1;
        }
        mp.sort_by(|a, b| b.cmp(a));
        let mut ans = 0;
        for (i, chunk) in mp.chunks(8).enumerate() {
            for ct in chunk {
                ans += ct * (i as i32 + 1);
            }
        }
        ans
    }
}
