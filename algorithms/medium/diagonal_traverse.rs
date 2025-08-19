impl Solution {
    pub fn find_diagonal_order(mat: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut m = mat.len() as i32;
        let mut n = mat[0].len() as i32;
        let total = m * n;
        let mut is_up = true;
        let mut i = 0;
        let mut j = 0;
        while ans.len() < (total as usize) {
            ans.push(mat[i as usize][j as usize]);
            if is_up {
                j += 1;
                if j == n {
                    j = n - 1;
                    i += 1;
                    is_up = false;
                } else {
                    i -= 1;
                    if i < 0 {
                        i = 0;
                        is_up = false;
                    }
                }
            } else {
                i += 1;
                if i == m {
                    i = m - 1;
                    j += 1;
                    is_up = true;
                } else {
                    j -= 1;
                    if j < 0 {
                        j = 0;
                        is_up = true;
                    }
                }
            }
        }

        ans
    }
}
