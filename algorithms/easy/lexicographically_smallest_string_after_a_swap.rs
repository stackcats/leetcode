impl Solution {
    pub fn get_smallest_string(mut s: String) -> String {
        unsafe {
            let bs = s.as_bytes_mut();
            let mut i = 0;
            for i in 0..bs.len() - 1 {
                if bs[i] % 2 == bs[i+1] % 2 && bs[i] > bs[i+1] {
                    bs.swap(i, i+1);
                    break;
                }
            }
            String::from_utf8_unchecked(bs.to_vec())
        }
    }
}
