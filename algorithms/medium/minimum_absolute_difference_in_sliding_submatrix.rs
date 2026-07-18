impl Solution {
    pub fn min_abs_diff(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let k = k as usize;

        let m = grid.len();
        let n = grid[0].len();

        let mut ans = vec![vec![0; n - k + 1]; m - k + 1];

        for i in 0..ans.len() {
            for j in 0..ans[0].len() {
                let mut arr = Vec::new();

                for x in i..i + k {
                    for y in j..j + k {
                        arr.push(grid[x][y]);
                    }
                }

                arr.sort_unstable();
                let mut t = i32::MAX;
                for x in 1..arr.len() {
                    if arr[x] != arr[x - 1] {
                        t = t.min(arr[x] - arr[x - 1]);
                    }
                }

                ans[i][j] = if t == i32::MAX { 0 } else { t };
            }
        }

        ans
    }
}
