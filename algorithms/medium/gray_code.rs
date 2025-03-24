impl Solution {
    pub fn gray_code(n: i32) -> Vec<i32> {
        let mut ans = vec![0, 1];
        for i in 1..n {
            let f = 1 << i;
            for j in (0..ans.len()).rev() {
                let m = ans[j] | f;
                ans.push(m);
            }
        }
        ans
    }
}
