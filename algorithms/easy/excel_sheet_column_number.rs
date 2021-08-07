impl Solution {
    pub fn title_to_number(s: String) -> i32 {
        let mut ans = 0;
        for c in s.chars() {
            let n = (c as i32) - 64;
            ans = ans * 26 + n;
        }
        ans
    }
}
