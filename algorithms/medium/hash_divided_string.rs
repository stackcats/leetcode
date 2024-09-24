impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let s = s.as_bytes();
        let k = k as usize;
        let mut res = String::new();
        for i in (0..s.len()).step_by(k) {
            let mut n = 0;
            for j in i..i + k {
                n += (s[j] - 97) as u32;
            }
            res.push(char::from_u32(n % 26 + 97).unwrap());
        }
        res
    }
}
