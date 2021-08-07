impl Solution {
    pub fn find_lu_slength(a: String, b: String) -> i32 {
        if a.len() != b.len() {
            return a.len().max(b.len()) as i32;
        }
        let mut i = 0;
        while i < a.len() && a.chars().nth(i) == b.chars().nth(i) {
            i += 1;
        }
        let ans = (a.len() - i) as i32;
        if ans == 0 {
            return -1;
        }
        ans
    }
}
