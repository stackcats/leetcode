impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let r = time / (n - 1);
        let m = time % (n - 1);
        if r % 2 == 0 {
            m
        } else {
            n - m
        }
    }
}
