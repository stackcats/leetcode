impl Solution {
    pub fn rotate_string(a: String, b: String) -> bool {
        if a.len() != b.len() {
            return false;
        }
        for i in 0..=a.len() {
            let rotated = format!("{}{}", &a[i..], &a[..i]);
            if rotated == b {
                return true;
            }
        }
        false
    }
}
