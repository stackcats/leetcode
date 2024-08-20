impl Solution {
    pub fn min_changes(n: i32, k: i32) -> i32 {
        if n & k == k {
            return (n ^ k).count_ones() as i32;
        }

        -1
    }
}
