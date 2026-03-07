impl Solution {
    pub fn min_flips(s: String) -> i32 {
        let n = s.len();
        let s = format!("{}{}", s, s).into_bytes();

        let mut diff1 = 0;
        let mut diff2 = 0;

        for i in 0..n {
            if s[i] != b'0' + (i % 2) as u8 {
                diff1 += 1;
            }
            if s[i] != b'0' + ((i + 1) % 2) as u8 {
                diff2 += 1;
            }
        }

        let mut ans = diff1.min(diff2);

        for i in n..2 * n {
            let j = i - n;

            if s[j] != b'0' + (j % 2) as u8 {
                diff1 -= 1;
            }
            if s[j] != b'0' + ((j + 1) % 2) as u8 {
                diff2 -= 1;
            }

            if s[i] != b'0' + (i % 2) as u8 {
                diff1 += 1;
            }
            if s[i] != b'0' + ((i + 1) % 2) as u8 {
                diff2 += 1;
            }

            ans = ans.min(diff1.min(diff2));
        }

        ans
    }
}
