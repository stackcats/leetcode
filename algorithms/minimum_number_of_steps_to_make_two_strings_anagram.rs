impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut s_count = vec![0; 26];
        for c in s.chars() {
            s_count[(c as u8 - b'a') as usize] += 1;
        }

        let mut t_count = vec![0; 26];
        for c in t.chars() {
            t_count[(c as u8 - b'a') as usize] += 1;
        }

        let mut ans = 0;

        for i in 0..26 {
            ans += 0.max(s_count[i] - t_count[i]);
        }
        ans
    }
}
