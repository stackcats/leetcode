use std::collections::VecDeque;

fn bfs(board: &mut Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>, r: usize, c: usize) {
    let dirs = [(1, 0), (0, -1), (-1, 0), (0, 1)];
    let mut q = VecDeque::new();
    q.push_back((r, c));
    while !q.is_empty() {
        let (mut m, mut n) = q.pop_front().unwrap();
        if visited[m][n] {
            continue;
        }
        visited[m][n] = true;
        for (dx, dy) in &dirs {
            let x = (m as i32) + dx;
            let y = (n as i32) + dy;
            if x >= 0
                && (x as usize) < board.len()
                && y >= 0
                && (y as usize) < board[0].len()
                && board[x as usize][y as usize] == 'O'
            {
                q.push_back((x as usize, y as usize));
            }
        }
    }
}

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        let m = board.len();
        let n = board[0].len();

        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            if board[i][0] == 'O' && !visited[i][0] {
                bfs(board, &mut visited, i, 0);
            }
            if board[i][n - 1] == 'O' && !visited[i][n - 1] {
                bfs(board, &mut visited, i, n - 1);
            }
        }

        for j in 0..n {
            if board[0][j] == 'O' && !visited[0][j] {
                bfs(board, &mut visited, 0, j);
            }
            if board[m - 1][j] == 'O' && !visited[m - 1][j] {
                bfs(board, &mut visited, m - 1, j);
            }
        }

        for i in 1..(m - 1) {
            for j in 1..(n - 1) {
                if board[i][j] == 'O' && !visited[i][j] {
                    board[i][j] = 'X';
                }
            }
        }
    }
}
