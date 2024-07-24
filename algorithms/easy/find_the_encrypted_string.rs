impl Solution {
    pub fn get_encrypted_string(s: String, k: i32) -> String {
        let mut ans = String::new();
        let s = s.as_bytes();
        for i in 0..s.len() {
            let t = s[(i + k as usize) % s.len()];
            ans.push(t as char);
        }
        ans
    }
}
