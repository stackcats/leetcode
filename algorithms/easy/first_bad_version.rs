// The API isBadVersion is defined for you.
// isBadVersion(versions:i32)-> bool;
// to call it use self.isBadVersion(versions)

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut r = std::i32::MAX;
        while l <= r {
            let m = (r - l) / 2 + l;
            if self.isBadVersion(m) {
                r = m - 1;
            } else {
                l = m + 1;
            }
        }
        r + 1
    }
}
