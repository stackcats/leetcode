impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut a = 0;
        let mut b = 0;
        for (i, c) in num.as_bytes().into_iter().enumerate() {
            if i % 2 == 0 {
                a += c - b'0';
            } else {
                b += c - b'0';
            }
        }
        a == b
    }
}
