impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let m = matrix.len();
        let n = matrix[0].len();

        if target < matrix[0][0] || target > matrix[m - 1][n - 1] {
            return false;
        }

        let mut left = 0;
        let mut right = m * n - 1;
        while left <= right {
            let mid = (right - left) / 2 + left;
            let i = mid / n;
            let j = mid % n;
            if matrix[i][j] == target {
                return true;
            }
            if matrix[i][j] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        false
    }
}
