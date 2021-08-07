impl Solution {
    pub fn find_the_winner(n: i32, k: i32) -> i32 {
        let mut s = 0;
        for i in 2..=n {
            s = (s + k) % i;
        }
        s + 1
    }
}
