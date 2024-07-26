impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut n = word.len() as i32;
        let mut ct = 1;
        let mut ans = 0;
        while n > 0 {
            ans += n.min(8) * ct;
            ct += 1;
            n -= 8;
        }

        ans
    }
}
