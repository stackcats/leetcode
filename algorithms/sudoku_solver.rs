fn is_valid_sudoku(board: &[Vec<char>], r: usize, c: usize, val: char) -> bool {
    // check row
    for j in 0..9 {
        if board[r][j] == val {
            return false;
        }
    }

    // check col
    for i in 0..9 {
        if board[i][c] == val {
            return false;
        }
    }

    // check sub box
    let r = r / 3 * 3;
    let c = c / 3 * 3;
    for i in 0..3 {
        for j in 0..3 {
            if board[r + i][c + j] == val {
                return false;
            }
        }
    }

    true
}

fn dfs(board: &mut Vec<Vec<char>>, q: &mut Vec<(usize, usize)>) -> bool {
    if q.is_empty() {
        return true;
    }

    let (i, j) = q.pop().unwrap();
    for c in '1'..='9' {
        if is_valid_sudoku(board, i, j, c) {
            board[i][j] = c;
            let is_finished = dfs(board, q);
            if is_finished {
                return true;
            }
        }
    }
    board[i][j] = '.';
    q.push((i, j));
    false
}

impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut q = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] == '.' {
                    q.push((i, j));
                }
            }
        }

        dfs(board, &mut q);
    }
}
