impl Solution {
    pub fn num_rook_captures(board: Vec<Vec<char>>) -> i32 {
        let mut ans = 0;
        for i in 0..8 {
            for j in 0..8 {
                if board[i][j] == 'R' {
                    let mut d = i + 1;
                    let mut can_capture = false;
                    while d < 8 {
                        if board[d][j] == 'p' {
                            can_capture = true;
                            break;
                        } else if board[d][j] == 'B' {
                            break;
                        }
                        d += 1;
                    }
                    if can_capture {
                        ans += 1;
                    }
                    let mut d = i as i32 - 1;
                    let mut can_capture = false;
                    while d >= 0 {
                        if board[d as usize][j] == 'p' {
                            can_capture = true;
                            break;
                        } else if board[d as usize][j] == 'B' {
                            break;
                        }
                        d -= 1;
                    }
                    if can_capture {
                        ans += 1;
                    }
                    let mut d = j + 1;
                    let mut can_capture = false;
                    while d < 8 {
                        if board[i][d] == 'p' {
                            can_capture = true;
                            break;
                        } else if board[i][d] == 'B' {
                            break;
                        }
                        d += 1;
                    }
                    if can_capture {
                        ans += 1;
                    }
                    let mut d = j as i32 - 1;
                    let mut can_capture = false;
                    while d >= 0 {
                        if board[i][d as usize] == 'p' {
                            can_capture = true;
                            break;
                        } else if board[i][d as usize] == 'B' {
                            break;
                        }
                        d -= 1;
                    }
                    if can_capture {
                        ans += 1;
                    }
                    break;
                }
            }
        }
        ans
    }
}
