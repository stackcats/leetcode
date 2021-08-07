fn is_valid(board: &[Vec<char>], r: usize, c: usize, size: usize) -> bool {
    for i in 0..r {
        if board[i][c] == 'Q' {
            return false;
        }
    }
    let r = r as i32;
    let c = c as i32;
    let size = size as i32;
    let mut dx = 1;
    while r - dx >= 0 && c + dx < size {
        if board[(r - dx) as usize][(c + dx) as usize] == 'Q' {
            return false;
        }
        dx += 1;
    }
    dx = 1;
    while r - dx >= 0 && c - dx >= 0 {
        if board[(r - dx) as usize][(c - dx) as usize] == 'Q' {
            return false;
        }
        dx += 1;
    }
    true
}

fn dfs(board: &mut Vec<Vec<char>>, curr_row: usize, size: usize, ans: &mut i32) {
    if curr_row >= size {
        *ans += 1;
        return;
    }

    for j in 0..size {
        if is_valid(board, curr_row, j, size) {
            board[curr_row][j] = 'Q';
            dfs(board, curr_row + 1, size, ans);
        }
        board[curr_row][j] = '.';
    }
}

impl Solution {
    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut board = vec![vec!['.'; n]; n];
        let mut ans = 0;
        dfs(&mut board, 0, n, &mut ans);
        ans
    }
}
