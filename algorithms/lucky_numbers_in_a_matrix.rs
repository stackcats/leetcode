impl Solution {
    pub fn lucky_numbers (matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = Vec::new();
        for i in 0..matrix.len() {
            let mut targetJ = 0;
            let mut smallest = matrix[i][0];
            for j in 1..matrix[i].len() {
                if matrix[i][j] < smallest {
                    targetJ = j;
                    smallest = matrix[i][j];
                }
            }
            
            let mut targetI = 0;
            let mut largest = matrix[0][targetJ];
            for k in 1..matrix.len() {
                if matrix[k][targetJ] > largest {
                    largest = matrix[k][targetJ];
                    targetI = k;
                }
            }
            if targetI == i {
                arr.push(matrix[targetI][targetJ]);
            }
        }
        arr
    }
}
