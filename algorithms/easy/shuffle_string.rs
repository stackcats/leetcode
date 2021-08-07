impl Solution {
    pub fn restore_string(s: String, indices: Vec<i32>) -> String {
        let bytes = s.as_bytes();
        let mut arr = vec![0; bytes.len()];
        for i in 0..indices.len() {
            arr[indices[i] as usize] = bytes[i];
        }
        String::from_utf8(arr).unwrap()
    }
}
