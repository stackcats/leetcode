impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        let mut ptr = 1;
        let word = word.as_bytes();
        let mut ans = 0;
        for w in word {
            let pos = (w - b'a' + 1) as i32;
            let diff1 = (pos - ptr).abs();
            let diff2 = (pos - ptr + 26).abs();
            let diff3 = (pos - ptr - 26).abs();
            let diff = diff1.min(diff2.min(diff3));
            ans += diff;
            ans += 1;
            ptr = pos;
        }
        ans
    }
}
