impl Solution {
    pub fn find_kth_positive(arr: Vec<i32>, k: i32) -> i32 {
        let mut n = 1;
        let mut i = 0;
        let mut x = 0;
        while i < arr.len() {
            if n < arr[i] {
                x += 1;
                if x == k {
                    return n;
                }
            } else if n == arr[i] {
                i += 1;
            }
            n += 1;
        }
        while x < k {
            n += 1;
            x += 1;
        }
        n - 1
    }
}
