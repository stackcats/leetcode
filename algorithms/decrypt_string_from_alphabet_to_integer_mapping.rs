impl Solution {
    pub fn freq_alphabets(s: String) -> String {
        let mut ans = Vec::new();
        let cs = s.chars().collect::<Vec<_>>();
        let mut i = 0;
        while i < s.len() {
            if i + 2 < s.len() && cs[i + 2] == '#' {
                let n = (cs[i] as u8 - 48) * 10 + (cs[i+1] as u8 - 48);
                ans.push((96 + n) as char);
                i += 3;
            } else {
                let n = cs[i] as u8 - 48;
                ans.push((96 + n) as char);
                i += 1;
            }
        }
        ans.iter().collect::<String>()
    }
}
