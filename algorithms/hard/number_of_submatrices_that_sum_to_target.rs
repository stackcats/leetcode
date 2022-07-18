use std::collections::HashMap;

impl Solution {
    pub fn num_submatrix_sum_target(mut matrix: Vec<Vec<i32>>, target: i32) -> i32 {
        for i in 0..matrix.len() {
            for j in 1..matrix[i].len() {
                matrix[i][j] += matrix[i][j-1];
            }
        }

        let mut ans = 0;
        for left in 0..matrix[0].len() {
            for right in left..matrix[0].len() {
                let mut map = HashMap::new();
                map.insert(0, 1);
                let mut pre_sum = 0;
                for r in 0..matrix.len() {
                    pre_sum += matrix[r][right];                        
                    if left > 0 {
                        pre_sum -= matrix[r][left-1];
                    }
                    ans += map.get(&(pre_sum- target)).unwrap_or(&0);
                    *map.entry(pre_sum).or_insert(0) += 1;
                }
            }
        }
        ans
    }
}
