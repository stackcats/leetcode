use std::collections::HashSet;

impl Solution {
    pub fn find_the_prefix_common_array(a: Vec<i32>, b: Vec<i32>) -> Vec<i32> {
        let mut sa = HashSet::new();
        let mut sb = HashSet::new();

        let mut ans = Vec::new();
        let mut ct = 0;
        for i in 0..a.len() {
            if a[i] == b[i] {
                ct += 1;
            } else {
                if sa.contains(&b[i]) {
                    ct += 1;
                }
                if sb.contains(&a[i]) {
                    ct += 1;
                }
            }
            sa.insert(a[i]);
            sb.insert(b[i]);

            ans.push(ct);
        }
        ans
    }
}
