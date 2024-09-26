impl Solution {
    pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
        let time = time % (n * 2 - 2);
        if time < n - 1 {
            time + 1
        } else {
            n - (time - n) - 1
        }
    }
}

