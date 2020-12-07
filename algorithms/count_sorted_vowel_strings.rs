impl Solution {
    pub fn count_vowel_strings(n: i32) -> i32 {
        let mut a = 1;
        let mut e = 1;
        let mut i = 1;
        let mut o = 1;
        let mut u = 1;

        for _ in 2..=n {
            a = a + e + i + o + u;
            e = e + i + o + u;
            i = i + o + u;
            o = o + u;
            u = u;
        }
        a + e + i + o + u
    }
}
