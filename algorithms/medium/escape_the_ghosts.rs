fn distance(a: &[i32], b: &[i32]) -> i32 {
    (a[0] - b[0]).abs() + (a[1] - b[1]).abs()
}

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let n = distance(&vec![0, 0], &target);
        ghosts.iter().map(|g| distance(g, &target)).all(|m| m > n)
    }
}
