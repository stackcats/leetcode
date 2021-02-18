impl Solution {
    pub fn diagonal_sort(mut mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        for r in (1..mat.len()).rev() {
            let mut i = 0;
            let mut arr = Vec::new();
            while r + i < mat.len() && i < mat[0].len() {
                arr.push(mat[r + i][i]);
                i += 1;
            }
            arr.sort();
            let mut i = 0;
            while r + i < mat.len() && i < mat[0].len() {
                mat[r + i][i] = arr[i];
                i += 1;
            }
        }
        for c in 0..mat[0].len() {
            let mut i = 0;
            let mut arr = Vec::new();
            while c + i < mat[0].len() && i < mat.len() {
                arr.push(mat[i][c + i]);
                i += 1;
            }
            arr.sort();
            let mut i = 0;
            while c + i < mat[0].len() && i < mat.len() {
                mat[i][c + i] = arr[i];
                i += 1;
            }
        }
        mat
    }
}
