impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        let mut ans = 0;
        for c in n.chars() {
            let d = c.to_digit(10).unwrap();
            ans = ans.max(d as i32);
        }
        ans
    }
}
