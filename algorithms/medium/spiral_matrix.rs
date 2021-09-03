impl Solution {
    pub fn spiral_order(matrix: Vec<Vec<i32>>) -> Vec<i32> {
        let mut ans = Vec::new();
        let mut top = 1;
        let mut bottom = matrix.len() - 1;
        let mut left = 0;
        let mut right = matrix[0].len() - 1;
        let mut dx = 0;
        let mut dy = 1;
        let dir = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let N = matrix.len() * matrix[0].len();
        let mut i = 0;
        let mut j = 0;
        let mut d = 0;
        while ans.len() < N {
            ans.push(matrix[i][j]);
            if d == 0 {
                if j == right {
                    right -= 1;
                    d = 1;
                }
            } else if d == 1 {
                if i == bottom {
                    bottom -= 1;
                    d = 2;
                }
            } else if d == 2 {
                if j == left {
                    left += 1;
                    d = 3;
                }
            } else {
                if i == top {
                    top += 1;
                    d = 0;
                }
            }

            let (dx, dy) = dir[d];
            i = ((i as i32) + dx) as usize;
            j = ((j as i32) + dy) as usize;
        }
        ans
    }
}
