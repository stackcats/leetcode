impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let mut ans = vec![vec![0; n as usize]; n as usize];
        let mut i = 0;
        let mut j = 0;
        let mut dx = 0;
        let mut dy = 1;
        let mut left = 0;
        let mut right = n - 1;
        let mut top = 1;
        let mut bottom = n - 1;
        for x in 1..=n * n {
            ans[i as usize][j as usize] = x;
            i += dx;
            j += dy;
            if dy == 1 && j == right {
                dx = 1;
                dy = 0;
                right -= 1;
            } else if dx == 1 && i == bottom {
                dx = 0;
                dy = -1;
                bottom -= 1;
            } else if dy == -1 && j == left {
                dx = -1;
                dy = 0;
                left += 1;
            } else if dx == -1 && i == top {
                dx = 0;
                dy = 1;
                top += 1;
            }
        }
        ans
    }
}
