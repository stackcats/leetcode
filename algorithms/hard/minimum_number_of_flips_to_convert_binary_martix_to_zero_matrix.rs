fn all_zeros(mat: &[Vec<i32>]) -> bool {
    for i in 0..mat.len() {
        for j in 0..mat[i].len() {
            if mat[i][j] == 1 {
                return false;
            }
        }
    }
    true
}

fn flip(mat: &mut Vec<Vec<i32>>, i: usize, j: usize) {
    for &(di, dj) in &[(0, 0), (-1, 0), (0, 1), (1, 0), (0, -1)] {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;
        if ni >= 0 && ni < (mat.len() as i32) && nj >= 0 && nj < (mat[i].len() as i32) {
            mat[ni as usize][nj as usize] ^= 1;
        }
    }
}

fn dfs(mat: &mut Vec<Vec<i32>>, mut i: usize, mut j: usize, steps: i32, ans: &mut i32) {
    if all_zeros(mat) {
        if *ans < 0 {
            *ans = steps;
        } else {
            *ans = (*ans).min(steps);
        }
        return;
    }
    

    if j == mat[i].len() {
        j = 0;
        i += 1;
    }
    
    if i == mat.len()  {
        return;
    }

    flip(mat, i, j);
    dfs(mat, i, j + 1, steps + 1, ans);
    flip(mat, i, j);
    dfs(mat, i, j + 1, steps, ans);
}

impl Solution {
    pub fn min_flips(mut mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = -1;        
        dfs(&mut mat, 0, 0, 0, &mut ans);
        ans
    }
}
