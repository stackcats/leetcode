use std::collections::HashSet;

fn dfs(board: &mut Vec<Vec<char>>, word: &[char], x: usize, y: usize) -> bool {
    if board[x][y] == '#' {
        return false;
    }

    if word[0] != board[x][y] {
        return false;
    }

    if word.len() == 1 {
        return true;
    }

    let c = board[x][y];

    board[x][y] = '#';

    let dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    for &(dx, dy) in &dirs {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && ny >= 0 && (nx as usize) < board.len() && (ny as usize) < board[0].len() {
            if dfs(board, &word[1..], nx as usize, ny as usize) {
                return true;
            }
        }
    }

    board[x][y] = c;

    false
}

impl Solution {
    pub fn exist(mut board: Vec<Vec<char>>, word: String) -> bool {
        let word: Vec<_> = word.chars().collect();

        if !board
            .iter()
            .flat_map(|row| row)
            .collect::<HashSet<&char>>()
            .is_superset(&word.iter().collect::<HashSet<&char>>())
        {
            return false;
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                if dfs(&mut board, &word, i, j) {
                    return true;
                }
            }
        }

        false
    }
}
