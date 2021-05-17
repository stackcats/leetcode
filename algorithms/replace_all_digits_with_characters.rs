impl Solution {
    pub fn replace_digits(s: String) -> String {
        let mut bs = s.as_bytes();
        let mut ans = vec![0; s.len()];
        for i in 0..s.len() {
            if i % 2 == 0 {
                ans[i] = bs[i];
            } else {
                ans[i] = bs[i - 1] + (bs[i] - b'0');
            }
        }
        std::str::from_utf8(&ans).unwrap().to_string()
    }
}
