impl Solution {
    pub fn count_battleships(board: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if board[i][j] == 'X' {
                    if i > 0 && board[i - 1][j] == 'X' {
                        continue;
                    }
                    if j > 0 && board[i][j - 1] == 'X' {
                        continue;
                    }
                    ans += 1;
                }
            }
        }
        ans
    }
}
