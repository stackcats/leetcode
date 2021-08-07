impl Solution {
    pub fn image_smoother(m: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut n = vec![vec![0; m[0].len()]; m.len()];
        let d = [
            (-1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1),
            (0, -1),
            (-1, -1),
        ];
        for i in 0..m.len() {
            for j in 0..m[i].len() {
                let mut sum = m[i][j];
                let mut nums = 1;
                for (dx, dy) in d.iter() {
                    let nx = (i as i32) + dx;
                    let ny = (j as i32) + dy;
                    if nx >= 0 && nx < m.len() as i32 && ny >= 0 && ny < m[i].len() as i32 {
                        sum += m[nx as usize][ny as usize];
                        nums += 1;
                    }
                }
                n[i][j] = sum / nums;
            }
        }
        n
    }
}
