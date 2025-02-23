impl Solution {
    pub fn has_same_digits(s: String) -> bool {
        let mut arr = Vec::new();
        for b in s.as_bytes() {
            arr.push(b - b'0');
        }
        while arr.len() > 2 {
            for i in 0..arr.len() - 1 {
                arr[i] = (arr[i] + arr[i + 1]) % 10;
            }
            arr.pop();
        }
        arr[0] == arr[1]
    }
}
