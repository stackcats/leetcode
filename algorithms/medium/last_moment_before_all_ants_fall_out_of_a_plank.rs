impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        let l = left.into_iter().max().unwrap_or_default();
        let r = right.into_iter().map(|x| n - x).max().unwrap_or_default();
        l.max(r)
    }
}
