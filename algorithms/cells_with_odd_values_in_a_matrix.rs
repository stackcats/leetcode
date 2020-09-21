impl Solution {
    pub fn odd_cells(n: i32, m: i32, indices: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let mut arr = vec![vec![0;m as usize]; n as usize];
        for indice in &indices {
            let ri = indice[0] as usize;
            let ci = indice[1] as usize;
           for j in 0..m {
               arr[ri][j as usize] += 1;
            } 
            for i in 0..n {
                arr[i as usize][ci] += 1;
            }
        }
        for i in 0..n {
            for j in 0..m {
                if arr[i as usize][j as usize] % 2 == 1 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
