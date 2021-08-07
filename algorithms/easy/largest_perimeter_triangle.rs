impl Solution {
    pub fn largest_perimeter(mut a: Vec<i32>) -> i32 {
        a.sort_by(|a, b| b.cmp(a));
        for i in 0..(a.len() - 2) {
            if a[i] < a[i + 1] + a[i + 2] {
                return a[i] + a[i + 1] + a[i + 2];
            }
        }
        0
    }
}
