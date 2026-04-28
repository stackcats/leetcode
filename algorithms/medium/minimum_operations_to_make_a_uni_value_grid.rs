impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut arr = Vec::new();
        let r = grid[0][0] % x;
        for row in grid {
            for n in row {
                if n % x != r {
                    return -1;
                }
                arr.push(n / x);
            }
        }

        arr.sort_unstable();
        let target = arr[arr.len() / 2];
        arr.into_iter().map(|n| (n - target).abs()).sum()
    }
}
