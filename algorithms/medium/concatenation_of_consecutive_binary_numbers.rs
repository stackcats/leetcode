impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut ans = 0i64;
        for i in 1..=(n as i64) {
            let bits = ((i as f64).log2()) as i64 + 1;
            ans = ((ans << bits) + i) % 1000000007;
        }
        ans as i32
    }
}
