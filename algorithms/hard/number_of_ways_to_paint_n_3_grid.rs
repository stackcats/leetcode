impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        let mut aba = 6i64;
        let mut abc = 6i64;
        let r = 1000000007i64;

        for _ in 1..n {
            (aba, abc) = ((3 * aba + 2 * abc) % r, (2 * aba + 2 * abc) % r);
        }

        ((aba + abc) % r) as _
    }
}
