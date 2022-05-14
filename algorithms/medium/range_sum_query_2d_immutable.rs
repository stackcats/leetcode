struct NumMatrix {
    mat: Vec<Vec<i32>>,
}


/** 
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl NumMatrix {

    fn new(matrix: Vec<Vec<i32>>) -> Self {
        let mut mat = vec![vec![0; matrix[0].len() + 1]; matrix.len() + 1];
        for i in 1..mat.len() {
            for j in 1..mat[0].len() {
                mat[i][j] = mat[i-1][j] + mat[i][j-1] - mat[i-1][j-1] + matrix[i-1][j-1];
            }
        }
        Self { mat }
    }
    
    fn sum_region(&self, row1: i32, col1: i32, row2: i32, col2: i32) -> i32 {
        let r1 = row1 as usize;
        let c1 = col1 as usize;
        let r2 = row2 as usize;
        let c2 = col2 as usize;
        self.mat[r2+1][c2+1] - self.mat[r2+1][c1] - self.mat[r1][c2+1] + self.mat[r1][c1]
    }
}

/**
 * Your NumMatrix object will be instantiated and called as such:
 * let obj = NumMatrix::new(matrix);
 * let ret_1: i32 = obj.sum_region(row1, col1, row2, col2);
 */
