impl Solution {
    pub fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
        let s1 = s1.as_bytes();
        let s2 = s2.as_bytes();
        let s3 = s3.as_bytes();
        let min_len = s1.len().min(s2.len()).min(s3.len());
        let mut i = 0;
        while i < min_len && s1[i] == s2[i] && s1[i] == s3[i] {
            i += 1;
        }
        if i == 0 {
            return -1;
        }
        (s1.len() - i + s2.len() - i + s3.len() - i) as i32
    }
}
