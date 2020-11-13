impl Solution {
    pub fn find_content_children(mut g: Vec<i32>, mut s: Vec<i32>) -> i32 {
        g.sort();
        s.sort();
        let mut i = 0;
        let mut j = 0;
        let mut ans = 0;
        while i < g.len() && j < s.len() {
            if g[i] > s[j] {
                j += 1;
            } else {
                i += 1;
                j += 1;
                ans += 1;
            }
        }
        ans
    }
}
