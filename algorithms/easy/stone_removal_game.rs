impl Solution {
    pub fn can_alice_win(mut n: i32) -> bool {
        let mut is_win = false;
        let mut stones = 10;
        while n >= stones {
            n -= stones;
            is_win = !is_win;
            stones -= 1;
        }
        is_win
    }
}
