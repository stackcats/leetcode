impl Solution {
    pub fn find_the_longest_substring(s: String) -> i32 {
        let mut mp = vec![-1; 32];
        let mut mask = 0;
        let mut res = 0;
        for (i, c) in s.chars().enumerate() {
            if let Some(j) = "aeiuo".find(c) {
                mask ^= 1 << j;
            }
            if mask > 0 && mp[mask] == -1 {
                mp[mask] = i as i32;
            }
            res = res.max(i as i32 - mp[mask]);
        }
        res
    }
}
