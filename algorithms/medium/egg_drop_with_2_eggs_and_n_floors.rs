impl Solution {
    pub fn two_egg_drop(mut n: i32) -> i32 {
        let mut i = 1;
        while n > i {
            n -= i;
            i += 1;
        }
        i
    }
}
