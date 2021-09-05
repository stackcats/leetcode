impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        let dirs = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
        ];
        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let ii = i as i32;
                let ij = j as i32;
                let mut lives = 0;
                for &(dx, dy) in &dirs {
                    let ni = ii + dx;
                    let nj = ij + dy;
                    if ni < 0
                        || ni >= (board.len() as i32)
                        || nj < 0
                        || nj >= (board[i].len() as i32)
                    {
                        continue;
                    }
                    if board[ni as usize][nj as usize] % 10 == 1 {
                        lives += 1;
                    }
                }
                if board[i][j] % 10 == 1 {
                    if lives == 2 || lives == 3 {
                        board[i][j] = 11;
                    }
                } else {
                    if lives == 3 {
                        board[i][j] = 10;
                    }
                }
            }
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                board[i][j] /= 10;
            }
        }
    }
}
