impl Solution {
    pub fn smallest_range_i(mut a: Vec<i32>, k: i32) -> i32 {
        a.sort();
        if a[0] + k >= a[a.len() - 1] - k {
            return 0;
        }
        a[a.len() - 1] - k - (a[0] + k)
    }
}
