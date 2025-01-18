impl Solution {
    pub fn zigzag_traversal(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let mut arr = Vec::new();
        let mut flag = true;
        let mut to_left = true;
        for i in 0..grid.len() {
            let r = 0..grid[0].len();
            let it: Box<dyn std::iter::Iterator<Item = usize>> = if to_left {
                Box::new(r)
            } else {
                Box::new(r.rev())
            };
            for j in it {
                if flag {
                    arr.push(grid[i][j]);
                }
                flag = !flag;
            }
            to_left = !to_left;
        }
        arr
    }
}
