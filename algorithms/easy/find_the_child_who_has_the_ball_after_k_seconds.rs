impl Solution {
    pub fn number_of_child(n: i32, k: i32) -> i32 {
        let n = n - 1;
        let q = k / n;
        let r = k % n;
        if q % 2 == 1 {
            n - r
        } else {
            r
        }
    }
}
