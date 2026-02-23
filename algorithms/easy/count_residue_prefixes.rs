impl Solution {
    pub fn residue_prefixes(s: String) -> i32 {
        let mut mp = vec![false; 26];
        let mut ans = 0;
        let mut dis = 0;
        let s = s.as_bytes();
        for i in 0..s.len() {
            let j = (s[i] - b'a') as usize;
            if !mp[j] {
                mp[j] = true;
                dis += 1;
            }
            if dis == (i + 1) % 3 {
                ans += 1;
            }
        }

        ans
    }
}
