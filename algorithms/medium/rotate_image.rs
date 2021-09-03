impl Solution {
    pub fn rotate(matrix: &mut Vec<Vec<i32>>) {
        let mut ax = 0;
        let mut by = matrix.len() - 1;
        let mut cx = matrix.len() - 1;
        let mut dy = 0;
        while ax < cx {
            let mut ay = ax;
            let mut bx = ax;
            let mut cy = matrix.len() - 1 - ax;
            let mut dx = matrix.len() - 1 - ax;
            while ay < by {
                let t = matrix[ax][ay];
                matrix[ax][ay] = matrix[dx][dy];
                matrix[dx][dy] = matrix[cx][cy];
                matrix[cx][cy] = matrix[bx][by];
                matrix[bx][by] = t;

                ay += 1;
                bx += 1;
                cy -= 1;
                dx -= 1;
            }
            ax += 1;
            by -= 1;
            cx -= 1;
            dy += 1;
        }
    }
}
