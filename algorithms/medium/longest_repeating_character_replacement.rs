impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut freq = [0; 26];
        let mut l = 0;
        let mut max_freq = 0;
        let mut ans = 0;

        for r in 0..s.len() {
            let i = (s[r] - b'A') as usize;
            freq[i] += 1;
            max_freq = max_freq.max(freq[i]);
            let len = r - l + 1;
            if max_freq + (k as usize) >= len {
                ans = ans.max(len);
            } else {
                freq[(s[l] - b'A') as usize] -= 1;
                l += 1;
            }
        }

        ans as _
    }
}
