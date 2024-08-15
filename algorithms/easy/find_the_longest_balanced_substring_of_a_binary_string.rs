impl Solution {
    pub fn find_the_longest_balanced_substring(s: String) -> i32 {
        let s = s.as_bytes();
        let mut zeros = if s[0] == b'0' { 1 } else { 0 };
        let mut ones = 1 - zeros;
        let mut ans = 0;
        for i in 1..s.len() {
            match (s[i-1], s[i]) {
                (b'0', b'0') => zeros += 1,
                (b'1', b'0') => {
                    ans = ans.max(zeros.min(ones));
                    ones = 0;
                    zeros = 1;
                },
                _ => ones += 1,
            }
        }
        ans.max(zeros.min(ones)) * 2
    }
}
