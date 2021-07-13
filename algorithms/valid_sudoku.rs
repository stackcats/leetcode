use std::collections::HashSet;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let len = 9;

        // check row
        for i in 0..len {
            let mut set = HashSet::new();
            for j in 0..len {
                if board[i][j] == '.' {
                    continue;
                }
                if set.contains(&board[i][j]) {
                    return false;
                }
                set.insert(board[i][j]);
            }
        }

        // check col
        for j in 0..len {
            let mut set = HashSet::new();
            for i in 0..len {
                if board[i][j] == '.' {
                    continue;
                }
                if set.contains(&board[i][j]) {
                    return false;
                }
                set.insert(board[i][j]);
            }
        }

        // check sub box
        let step = 3;
        for r in (0..len).step_by(step) {
            for c in (0..len).step_by(step) {
                let mut set = HashSet::new();
                for i in 0..step {
                    for j in 0..step {
                        if board[r + i][c + j] == '.' {
                            continue;
                        }
                        if set.contains(&board[r + i][c + j]) {
                            return false;
                        }
                        set.insert(board[r + i][c + j]);
                    }
                }
            }
        }
        true
    }
}
