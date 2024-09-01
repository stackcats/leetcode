fn is_black(c: &[u8]) -> bool {
    (c[0] + c[1]) % 2 == 0
}

impl Solution {
    pub fn check_two_chessboards(coordinate1: String, coordinate2: String) -> bool {
        let c1 = coordinate1.as_bytes();
        let c2 = coordinate2.as_bytes();
        is_black(c1) == is_black(c2)
    }
}
