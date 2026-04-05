impl Solution {
    pub fn mirror_frequency(s: String) -> i32 {
        let mut freq = vec![0i32; 255];
        for b in s.into_bytes() {
            freq[b as usize] += 1;
        }

        let mut ans = 0;

        for i in 0..13 {
            ans += (freq[(b'a' + i) as usize] - freq[(b'z' - i) as usize]).abs();
        }
        for i in 0..5 {
            ans += (freq[(b'0' + i) as usize] - freq[(b'9' - i) as usize]).abs();
        }

        ans
    }
}
