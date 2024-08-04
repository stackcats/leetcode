impl Solution {
    pub fn can_be_equal(mut s1: String, mut s2: String) -> bool {
        let s1 = unsafe { s1.as_bytes_mut() };
        let s2 = unsafe { s2.as_bytes_mut() };
        if s1 == s2 {
            return true
        }

        s1.swap(0, 2);
        if s1 == s2 {
            return true;
        }

        s1.swap(1, 3);
        if s1 == s2 {
            return true;
        }

        s1.swap(0, 2);
        if s1 == s2 {
            return true;
        }

        false
    }
}
