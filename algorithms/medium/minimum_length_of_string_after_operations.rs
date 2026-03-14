impl Solution {
    pub fn minimum_length(s: String) -> i32 {
        let mut mp = vec![0; 26];
        for c in s.as_bytes() {
            mp[(c - b'a') as usize] += 1;
        }
        let mut ans = 0;
        for i in 0..mp.len() {
            if mp[i] == 0 {
                continue;
            }
            ans += if mp[i] % 2 == 1 { 1 } else { 2 };
        }
        ans
    }
}
