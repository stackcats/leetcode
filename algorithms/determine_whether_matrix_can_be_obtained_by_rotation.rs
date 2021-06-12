impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut is_equal = true;
        for i in 0..mat.len() {
            for j in 0..mat[i].len() {
                if mat[i][j] != target[i][j] {
                    is_equal = false;
                }
            }
        }

        if is_equal {
            return true;
        }

        is_equal = true;
        let mut n = mat[0].len() - 1;
        for i in 0..mat.len() {
            let mut m = 0;
            for j in 0..mat[i].len() {
                if mat[i][j] != target[m][n] {
                    is_equal = false;
                }
                m += 1;
            }
            n -= 1;
        }

        if is_equal {
            return true;
        }

        is_equal = true;

        let mut m = mat.len() - 1;
        for i in 0..mat.len() {
            let mut n = mat[i].len() - 1;
            for j in 0..mat[i].len() {
                if mat[i][j] != target[m][n] {
                    is_equal = false;
                }
                n -= 1;
            }
            m -= 1;
        }

        if is_equal {
            return true;
        }

        is_equal = true;

        let mut m = mat.len() - 1;
        for i in 0..mat.len() {
            let mut n = mat[i].len() - 1;
            for j in 0..mat[i].len() {
                if mat[i][j] != target[m][n] {
                    is_equal = false;
                }
                n -= 1;
            }
            m -= 1;
        }

        if is_equal {
            return true;
        }

        is_equal = true;

        let mut n = 0;
        for i in 0..mat.len() {
            let mut m = mat.len() - 1;
            for j in 0..mat[i].len() {
                if mat[i][j] != target[m][n] {
                    is_equal = false;
                }
                m -= 1;
            }
            n += 1;
        }

        if is_equal {
            return true;
        }

        false
    }
}
