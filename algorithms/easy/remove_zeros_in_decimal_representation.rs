impl Solution {
    pub fn remove_zeros(n: i64) -> i64 {
        let s = format!("{}", n);
        s.replace("0", "").parse::<i64>().unwrap()
    }
}
